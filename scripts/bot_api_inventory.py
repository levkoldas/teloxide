#!/usr/bin/env python3
"""Snapshot the official Bot API tables for coverage review (Python stdlib only).

Usage: python3 scripts/bot_api_inventory.py [--html downloaded-api.html]
The snapshot is evidence for review, not a claim that the Rust API covers it.
"""
import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parents[1]
URL = "https://core.telegram.org/bots/api"


class Node:
    def __init__(self, tag="", attrs=()):
        self.tag, self.attrs, self.children = tag, dict(attrs), []

    def text(self):
        return "".join(c.text() if isinstance(c, Node) else c for c in self.children)

    def find(self, tag):
        for child in self.children:
            if isinstance(child, Node):
                if child.tag == tag:
                    yield child
                yield from child.find(tag)


class Document(HTMLParser):
    VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}

    def __init__(self, html):
        super().__init__()
        self.root = Node()
        self.stack = [self.root]
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        node = Node(tag, attrs)
        self.stack[-1].children.append(node)
        if tag not in self.VOID:
            self.stack.append(node)

    def handle_endtag(self, tag):
        for i in range(len(self.stack) - 1, 0, -1):
            if self.stack[i].tag == tag:
                del self.stack[i:]
                return

    def handle_data(self, data):
        self.stack[-1].children.append(data)


def snapshot(html):
    root = Document(html).root
    article = next(n for n in root.find("div") if n.attrs.get("id") == "dev_page_content")
    sections = {}
    current = None
    for node in article.children:
        if not isinstance(node, Node):
            continue
        if node.tag in {"h3", "h4"}:
            name = node.text().strip()
            current = {"description": [], "fields": [], "variants": []} if node.tag == "h4" and re.fullmatch(r"[A-Za-z][A-Za-z0-9]*", name) else None
            if current is not None:
                sections[name] = current
        elif current is not None:
            if node.tag == "p":
                current["description"].append(node.text().strip())
            elif node.tag == "table":
                for row in node.find("tr"):
                    cells = [c.text().strip() for c in row.find("td")]
                    if not cells:
                        continue
                    if len(cells) not in (3, 4):
                        raise ValueError(f"Unrecognized table row in {name}: {cells}")
                    field, ty = cells[:2]
                    desc = cells[-1]
                    required = cells[2] == "Yes" if len(cells) == 4 else not desc.startswith("Optional.")
                    current["fields"].append(dict(name=field, type=ty, required=required, description=desc))
            elif node.tag == "ul":
                current["variants"].extend(a.text() for a in node.find("a") if re.fullmatch(r"[A-Z][a-zA-Z0-9]*", a.text()))
    version = re.search(r"Bot API (\d+\.\d+)", article.text()).group(1)
    methods = {k: v for k, v in sections.items() if k[0].islower()}
    types = {k: v for k, v in sections.items() if k[0].isupper()}
    # Reject truncated/error responses instead of silently replacing the inventory.
    assert len(methods) >= 160 and len(types) >= 300, (len(methods), len(types))
    return dict(source=URL, version=version, source_sha256=hashlib.sha256(html.encode()).hexdigest(), methods=methods, types=types)



def kind_name(kind):
    """Normalize the existing validation schema to official table spelling."""
    tag = kind["type"]
    if tag == "reference":
        return kind["reference"]
    if tag == "array":
        return "Array of " + kind_name(kind["array"])
    if tag == "any_of":
        return " or ".join(sorted(kind_name(k) for k in kind["any_of"]))
    return {"integer": "Integer", "string": "String", "bool": "Boolean",
            "float": "Float", "null": "Null"}[tag]


def normalized_type(name):
    if name.startswith("Array of "):
        item = name[9:]
        if ", " in item and " and " in item:
            item = item.replace(", ", " or ").replace(" and ", " or ")
        return "Array of (" + normalized_type(item) + ")"
    return " or ".join(sorted(name.replace("True", "Boolean").replace("Float number", "Float").split(" or ")))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path)
    parser.add_argument("--output", type=Path, default=ROOT / "crates/teloxide-core/bot-api-latest.json")
    args = parser.parse_args()
    html = args.html.read_text() if args.html else urlopen(URL, timeout=60).read().decode()
    data = snapshot(html)
    args.output.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    baseline = json.loads((ROOT / "crates/teloxide-core/custom_v2.json").read_text())
    print(f"Bot API {data['version']}: {len(data['methods'])} methods, {len(data['types'])} types")
    for key, old_key in [("methods", "methods"), ("types", "objects")]:
        old = {x['name']: x for x in baseline[old_key]}
        new = data[key]
        print(f"\n{key} added since checked-in schema: {', '.join(sorted(new.keys() - old.keys()))}")
        for name in sorted(old.keys() & new.keys()):
            old_fields = {f['name'] for f in old[name].get('arguments' if key == 'methods' else 'properties', [])}
            new_fields = {f['name'] for f in new[name]['fields']}
            added, removed = new_fields - old_fields, old_fields - new_fields
            if added or removed:
                print(f"  {name}: added {', '.join(sorted(added)) or '-'}; removed {', '.join(sorted(removed)) or '-'}")
            previous = {f['name']: f for f in old[name].get('arguments' if key == 'methods' else 'properties', [])}
            for field in new[name]['fields']:
                prior = previous.get(field['name'])
                if prior is None:
                    continue
                before = (normalized_type(kind_name(prior['type_info'])), prior['required'])
                after = (normalized_type(field['type']), field['required'])
                if before != after:
                    print(f"  {name}.{field['name']}: type/required {before} -> {after}")
            if old[name].get('type') == 'any_of':
                prior = {k['reference'] for k in old[name]['any_of'] if k['type'] == 'reference'}
                current = set(new[name]['variants'])
                if prior != current:
                    print(f"  {name}: union added {sorted(current - prior)}; removed {sorted(prior - current)}")


if __name__ == "__main__":
    main()
