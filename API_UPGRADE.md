# Bot API and dependency upgrade

Target: Telegram Bot API **10.3** (August 24, 2026), researched November 1, 2026.
The API implementation, schema coverage, dependency refresh, and local validation
are complete.

## Authoritative sources

- [Current Bot API reference](https://core.telegram.org/bots/api)
- [Official changelog](https://core.telegram.org/bots/api-changelog) (9.3 through 10.3)
- `crates/teloxide-core/bot-api-latest.json`: snapshot of 185 method tables and
  400 type definitions, including descriptions, field types, optionality and
  union members. Source URL, version and HTML SHA-256 are recorded in the file.
- `python3 scripts/bot_api_inventory.py` refreshes the snapshot and reports field
  and method differences, type/optionality changes and union-member differences
  against the existing validation schema. It can also
  consume a saved official page using `--html`. This inventory is review evidence,
  not a verifier of complete Rust coverage. Constraints, enum values, optionality
  and changes to existing descriptions also require review.

## Dependency work

All direct dependency requirements were checked against the crates.io registry
and refreshed, including major upgrades of SQLx, deadpool-redis, rand, syn,
RON, tower-http and bincode. The lockfile was re-resolved. Minimum supported Rust
is now 1.96 (required by takecell 0.1.2); the formatter/nightly CI pin is
`nightly-2026-09-01`. CI actions were also refreshed from upstream releases.

Bincode 3.0.0 is deliberately excluded: its published `src/lib.rs` consists only
of `compile_error!("https://xkcd.com/2347/")`. The latest usable release, 2.0.1,
is used with Serde and legacy configuration to preserve existing stored dialogue
bytes. A fixed fixture tests encoding, decoding, accepted trailing bytes and
rejection of truncated input. The public error type is now `BincodeError`.

## API implementation checklist

- [x] Inventory all current official methods and types.
- [x] Add `EphemeralMessageParameters` and the parameter to the twelve existing
  send methods that accept it plus `sendLivePhoto` and `sendRichMessage`.
- [x] 9.3: private-chat topics and streaming drafts; gifts and resale changes;
  user ratings, story reposting, forwarding effects and checklist completion.
- [x] 9.4: button styles/icons, ownership service messages, bot profile photos,
  video qualities and profile audios, unique-gift rarity/burning.
- [x] 9.5: date/time entities, member tags and their permissions.
- [x] 9.6: managed bots, prepared keyboard buttons, expanded polls and option
  service messages/replies.
- [x] 10.0: guest queries, reaction deletion, poll media, live photos and paid
  live photos, bot access settings and personal-chat messages.
- [x] 10.1: rich text, blocks and messages (including inline content, drafts,
  edit requests and nested file uploads); join-request queries/web apps; links.
- [x] 10.2: input rich blocks/media; ephemeral IDs, replies, edit/delete methods
  and bot commands; communities; subscription updates.
- [x] 10.3: rich buttons/documents/expandable quotations and compact tables;
  ephemeral edit uploads; welcome-message rights; disabled buttons and
  force-reply keyboards; stoppable drafts and generation-stopped updates;
  community joins and unique-gift message fields.
- [x] Regenerate and verify all payloads, requester/adaptor surfaces, multipart
  handling, update dispatch filters and allowed-update flags.
- [x] Update validation schema/version, documentation, migration guide and
  coverage badge against all 185 methods and 400 types.
- [x] Complete workspace tests, docs/examples, feature matrix, storage integration,
  MSRV, formatting and lint checks on the final tree.

Initially 27 methods were missing: `answerChatJoinRequestQuery`,
`answerGuestQuery`, `deleteAllMessageReactions`, `deleteEphemeralMessage`,
`deleteMessageReaction`, `editEphemeralMessageCaption`, `editEphemeralMessageMedia`,
`editEphemeralMessageReplyMarkup`, `editEphemeralMessageText`, `getChatGifts`,
`getManagedBotAccessSettings`, `getManagedBotToken`, `getUserGifts`,
`getUserPersonalChatMessages`, `getUserProfileAudios`, `removeMyProfilePhoto`,
`replaceManagedBotToken`, `repostStory`, `savePreparedKeyboardButton`,
`sendChatJoinRequestWebApp`, `sendLivePhoto`, `sendMessageDraft`, `sendRichMessage`,
`sendRichMessageDraft`, `setChatMemberTag`, `setManagedBotAccessSettings`,
`setMyProfilePhoto`.

## Bot API 10.3 implementation

- All 27 formerly missing methods are available through `Bot`, `Requester`,
  erased requests, caching, tracing, throttling, and default parse-mode adapters.
- Added the complete input/output rich-text and block families, nested media
  uploads, live photos and paid live photos, poll media and persistent options,
  all new service messages, update kinds, dispatch filters and allowed-update flags.
- Updated existing gift, chat, user, keyboard, permission, entity, reply and
  request fields. Poll closure accepts the new 2,628,000-second maximum.
- Fixed owned-JSON update decoding and shared attachment identifiers for inline
  result serialization; changing a cloned upload's filename gives it a distinct
  attachment identifier. Message decoding grows the stack for nested payloads.
- Poll media uses a single optional tagged variant, reducing its size and
  preventing construction of multiple simultaneous media values.
- Optional bot usernames no longer panic in command filters or `Me` helpers.
- Reply helpers preserve ephemeral message IDs, inline edit methods expose all
  current parameters, and game scores accept full-width signed chat IDs.
- Default parse modes propagate through nested rich-message and poll media
  without replacing explicit text entities.
- `MIGRATION_GUIDE.md` documents source changes and refreshed storage error types.
- The inventory reports no method, field, type, optionality, or union differences.
  Rust schema and RON method checks pass against the complete official snapshot.

## Final validation

- Stable and pinned-nightly workspace test suites each pass 369 tests with
  `full` (plus `nightly` on the nightly run). The six service-dependent tests
  pass separately against isolated Redis and PostgreSQL instances, for **375
  passing tests**. SQLite tests are included in the main suite.
- All **62 documentation tests** pass.
- Workspace/all-targets compilation with `full` passes on Rust **1.96**.
- Builds pass with no default features and with each of rustls and native TLS,
  including their SQLite/PostgreSQL storage features, macros, and Axum webhooks.
- Documentation builds with `RUSTDOCFLAGS="-D warnings"`.
- Workspace/all-targets Clippy with `full nightly` and `-D warnings` passes.
- `cargo fmt --all --check` and `git diff --check` pass.
- Inventory comparison, Rust type-schema checks, RON method-schema checks, and
  generated-source consistency checks pass.

Validation uses the official API snapshot and local tests; live Telegram API
calls were not made.

The latest nightly (`2026-11-01`) fails with rustc E0391 while computing
async-drop requirements for recursive `MessageKind`/`MessageCommon`.
`nightly-2026-09-01` is pinned and passes the checks above.

The dependency `proc-macro-error2` 2.0.1 emits an upstream future-compatibility
warning about re-exporting private `proc_macro`; this is not a project warning.
No newer crates.io release was available during the dependency audit.
