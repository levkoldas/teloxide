use std::collections::BTreeSet;

use serde_json::{json, Value};

use crate::{payloads::*, requests::MultipartPayload, types::*};

fn user(id: u64) -> Value {
    json!({"id": id, "is_bot": false, "first_name": "Ada"})
}

fn message() -> Value {
    json!({"message_id": 7, "date": 1, "chat": {"id": 1, "type": "private", "first_name": "Ada", "last_name": null, "username": null}})
}

fn mention(id: u64) -> Value {
    json!({"type": "text_mention", "text": "Ada", "user": user(id)})
}

#[test]
fn rich_text_preserves_plain_concat_and_nested_formatting() {
    let wire = json!(["Hi ", {"type": "bold", "text": [mention(2), "!"]}, {
        "type": "date_time", "text": "tomorrow", "unix_time": 4_300_000_000_i64,
        "date_time_format": "r"
    }]);
    let text: RichText = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(&text).unwrap(), wire);
    assert_eq!(text.mentioned_users().map(|u| u.id.0).collect::<Vec<_>>(), [2]);
    assert!(serde_json::from_value::<RichText>(json!({"type": "unknown", "text": "x"})).is_err());
}

#[test]
fn rich_message_walks_tables_quotations_and_captions() {
    let rich = json!({"blocks": [
        {"type": "heading", "text": "Heading", "size": 2},
        {"type": "table", "cells": [[{"text": mention(2), "align": "left", "valign": "top"}]], "is_compact": true},
        {"type": "expandable_blockquote", "text": mention(3), "credit": mention(4)}
    ]});
    let mut wire = message();
    wire["rich_message"] = rich.clone();
    let parsed: Message = serde_json::from_value(wire).unwrap();
    assert_eq!(serde_json::to_value(parsed.rich_message().unwrap()).unwrap(), rich);
    assert_eq!(parsed.mentioned_users().map(|u| u.id.0).collect::<Vec<_>>(), [2, 3, 4]);
}

#[test]
fn live_photo_messages_keep_captions_and_media_helpers() {
    let mut wire = message();
    wire["live_photo"] = json!({"file_id": "video", "file_unique_id": "unique", "width": 640,
        "height": 480, "duration": 3, "file_size": 5_000_000_000_u64});
    wire["caption"] = json!("A moving photo");
    wire["media_group_id"] = json!("album");
    wire["has_media_spoiler"] = json!(true);
    wire["show_caption_above_media"] = json!(true);
    let parsed: Message = serde_json::from_value(wire).unwrap();
    assert_eq!(parsed.live_photo().unwrap().file_size, Some(5_000_000_000));
    assert_eq!(parsed.caption(), Some("A moving photo"));
    assert!(parsed.has_media_spoiler());
    assert!(parsed.show_caption_above_media());
    assert!(parsed.media_group_id().is_some());
}

#[test]
fn guest_update_retains_receiver_and_caller_metadata() {
    let mut guest = message();
    guest["text"] = json!("hello");
    guest["from"] = user(2);
    guest["receiver_user"] = user(3);
    guest["guest_bot_caller_user"] = user(4);
    guest["ephemeral_message_id"] = json!(4_300_000_000_i64);
    guest["guest_query_id"] = json!("query");
    guest["sender_tag"] = json!("maintainer");
    guest["reply_to_poll_option_id"] = json!("persistent");
    let update: Update =
        serde_json::from_value(json!({"update_id": 10, "guest_message": guest})).unwrap();
    assert!(matches!(update.kind, UpdateKind::GuestMessage(_)));
    assert_eq!(update.from().unwrap().id, UserId(2));
    assert_eq!(update.chat().unwrap().id, ChatId(1));
    assert_eq!(update.mentioned_users().map(|u| u.id.0).collect::<Vec<_>>(), [2, 3, 4]);
    let encoded = serde_json::to_value(update).unwrap();
    assert_eq!(encoded["guest_message"]["ephemeral_message_id"], 4_300_000_000_i64);
    assert_eq!(encoded["guest_message"]["reply_to_poll_option_id"], "persistent");
}

#[test]
fn new_update_variants_have_correct_wire_names_and_projections() {
    for (key, data, from, chat) in [
        ("managed_bot", json!({"user": user(2), "bot": user(3)}), Some(2), None),
        (
            "subscription",
            json!({"user": user(2), "invoice_payload": "order", "state": "active"}),
            Some(2),
            None,
        ),
        (
            "stopped_message_generation",
            json!({"chat": message()["chat"], "draft_id": 4_300_000_000_i64}),
            None,
            Some(1),
        ),
    ] {
        let wire = json!({"update_id": 10, key: data});
        let update: Update = serde_json::from_value(wire.clone()).unwrap();
        assert!(!matches!(update.kind, UpdateKind::Error(_)), "{key}");
        assert_eq!(update.from().map(|u| u.id.0), from, "{key}");
        assert_eq!(update.chat().map(|c| c.id.0), chat, "{key}");
        assert_eq!(serde_json::to_value(update).unwrap(), wire);
        let allowed: AllowedUpdate = serde_json::from_value(json!(key)).unwrap();
        assert_eq!(serde_json::to_value(allowed).unwrap(), key);
    }
}

#[test]
fn new_service_messages_are_not_discarded_as_empty() {
    let community = json!({"id": 4_300_000_000_i64, "name": "Developers"});
    let gift = json!({"gift": {"id": "gift", "sticker": {"file_id": "file", "file_unique_id": "unique", "type": "regular", "width": 1, "height": 1, "is_animated": false, "is_video": false}, "star_count": 10}});
    for (key, content) in [
        ("chat_owner_left", json!({})),
        ("chat_owner_changed", json!({"new_owner": user(2)})),
        ("gift_upgrade_sent", gift),
        ("community_chat_added", json!({"community": community})),
        ("community_chat_joined", json!({"community": community})),
        ("community_chat_removed", json!({})),
        ("managed_bot_created", json!({"bot": user(2)})),
        ("poll_option_added", json!({"option_persistent_id": "x", "option_text": "new"})),
        ("poll_option_deleted", json!({"option_persistent_id": "x", "option_text": "old"})),
    ] {
        let mut wire = message();
        wire[key] = content.clone();
        let parsed: Message = serde_json::from_value(wire).unwrap();
        assert!(!matches!(parsed.kind, MessageKind::Empty { .. }), "{key}");
        // Existing media types can serialize omitted false fields; the service
        // payload must survive.
        let encoded = serde_json::to_value(parsed).unwrap();
        assert!(encoded.get(key).is_some(), "{key}");
    }
}

fn attachment_ids(value: &Value, ids: &mut BTreeSet<String>) {
    match value {
        Value::String(s) if s.starts_with("attach://") => {
            ids.insert(s[9..].into());
        }
        Value::Array(items) => items.iter().for_each(|v| attachment_ids(v, ids)),
        Value::Object(fields) => fields.values().for_each(|v| attachment_ids(v, ids)),
        _ => {}
    }
}

fn assert_files_match_wire<T: MultipartPayload + serde::Serialize>(mut payload: T, count: usize) {
    let wire = serde_json::to_value(&payload).unwrap();
    let mut expected = BTreeSet::new();
    attachment_ids(&wire, &mut expected);
    assert_eq!(expected.len(), count);
    let mut copied = BTreeSet::new();
    payload.copy_files(&mut |file| {
        if file.needs_attach() {
            copied.insert(file.id().into());
        }
    });
    assert_eq!(copied, expected);
    let mut moved = BTreeSet::new();
    payload.move_files(&mut |file| {
        if file.needs_attach() {
            moved.insert(file.id().into());
        }
    });
    assert_eq!(moved, expected);
}

fn upload() -> InputFile {
    InputFile::memory(vec![1, 2, 3])
}

fn rich_upload() -> InputRichMessage {
    InputRichMessage::new().blocks(vec![InputRichBlockDetails::new(
        "Details".into(),
        vec![
            InputRichBlockVideo::new(
                InputMediaVideo::new(upload()).thumbnail(upload()).cover(upload()),
            )
            .into(),
            InputRichBlockPhoto::new(InputMediaPhoto::new(upload())).into(),
        ],
    )
    .into()])
}

#[test]
fn rich_media_has_nested_discriminators_and_collects_every_upload() {
    let rich = rich_upload();
    let wire = serde_json::to_value(&rich).unwrap();
    assert_eq!(wire["blocks"][0]["blocks"][0]["type"], "video");
    assert_eq!(wire["blocks"][0]["blocks"][0]["video"]["type"], "video");
    assert_eq!(wire["blocks"][0]["blocks"][1]["photo"]["type"], "photo");
    assert_files_match_wire(SendRichMessage::new(ChatId(1), rich), 4);
    assert_files_match_wire(
        EditEphemeralMessageText::new(ChatId(1), UserId(2), 3).rich_message(rich_upload()),
        4,
    );
    assert_files_match_wire(
        EditMessageText::new(ChatId(1), MessageId(7)).rich_message(rich_upload()),
        4,
    );
}

#[test]
fn live_photos_poll_media_and_video_covers_collect_uploads() {
    assert_files_match_wire(SendLivePhoto::new(ChatId(1), upload(), upload()), 2);
    assert_files_match_wire(
        SendPaidMedia::new(
            ChatId(1),
            25_000,
            [InputPaidMedia::LivePhoto(InputPaidMediaLivePhoto::new(upload(), upload()))],
        ),
        2,
    );
    let live = InputMediaLivePhoto::new(upload(), upload());
    assert_files_match_wire(SendMediaGroup::new(ChatId(1), [InputMedia::LivePhoto(live)]), 2);
    assert_files_match_wire(
        EditEphemeralMessageMedia::new(
            ChatId(1),
            UserId(2),
            3,
            InputMedia::Video(InputMediaVideo::new(upload()).cover(upload()).thumbnail(upload())),
        ),
        3,
    );
    let option = InputPollOption::new("choice").media(InputMediaPhoto::new(upload()).into());
    assert_files_match_wire(
        SendPoll::new(ChatId(1), "Question", [option])
            .media(InputMediaLivePhoto::new(upload(), upload()).into()),
        3,
    );
}

#[test]
fn profile_photo_and_inline_rich_results_collect_uploads() {
    assert_files_match_wire(
        SetMyProfilePhoto::new(InputProfilePhoto::Static(InputProfilePhotoStatic {
            photo: upload(),
        })),
        1,
    );
    let content = InputMessageContent::RichMessage(InputRichMessageContent::new(rich_upload()));
    let result = InlineQueryResult::Article(InlineQueryResultArticle::new("id", "title", content));
    assert_files_match_wire(AnswerInlineQuery::new(InlineQueryId("query".into()), [result]), 4);
}

#[test]
fn ephemeral_replies_omit_normal_message_id_and_keep_large_identifier() {
    let reply = ReplyParameters::default()
        .ephemeral_message_id(4_300_000_000_i64)
        .poll_option_id("option".into());
    assert_eq!(
        serde_json::to_value(&reply).unwrap(),
        json!({"ephemeral_message_id": 4_300_000_000_i64, "poll_option_id": "option"})
    );
    assert_eq!(
        serde_json::to_value(ReplyParameters::new(MessageId(7))).unwrap(),
        json!({"message_id": 7})
    );
    assert_eq!(
        serde_json::from_value::<ReplyParameters>(serde_json::to_value(&reply).unwrap()).unwrap(),
        reply
    );
}

#[test]
fn keyboards_support_disabled_buttons_styles_icons_and_force_reply() {
    let button = InlineKeyboardButton::disabled("Unavailable", DisabledButton::new())
        .style(ButtonStyle::Danger)
        .icon_custom_emoji_id(CustomEmojiId("emoji".into()));
    let markup = InlineKeyboardMarkup::new([[button]]).force_reply(true);
    let wire = json!({"inline_keyboard": [[{"text": "Unavailable", "disabled": {}, "style": "danger", "icon_custom_emoji_id": "emoji"}]], "force_reply": true});
    assert_eq!(serde_json::to_value(&markup).unwrap(), wire);
    assert_eq!(serde_json::from_value::<InlineKeyboardMarkup>(wire).unwrap(), markup);
    let keyboard: KeyboardButton = serde_json::from_value(
        json!({"text": "Create bot", "request_managed_bot": {"request_id": 5}}),
    )
    .unwrap();
    assert!(matches!(keyboard.request, Some(ButtonRequest::ManagedBot(_))));
}

#[test]
fn new_permissions_and_date_time_entities_roundtrip() {
    let permissions = ChatPermissions::REACT_TO_MESSAGES | ChatPermissions::EDIT_TAG;
    let encoded = serde_json::to_value(&permissions).unwrap();
    assert_eq!(encoded["can_react_to_messages"], true);
    assert_eq!(encoded["can_edit_tag"], true);
    assert_eq!(serde_json::from_value::<ChatPermissions>(encoded).unwrap(), permissions);
    let entity = MessageEntity::date_time(4_300_000_000, Some("r".into()), 2, 5);
    let wire = json!({"type": "date_time", "unix_time": 4_300_000_000_i64, "date_time_format": "r", "offset": 2, "length": 5});
    assert_eq!(serde_json::to_value(&entity).unwrap(), wire);
    assert_eq!(serde_json::from_value::<MessageEntity>(wire).unwrap(), entity);
}

#[test]
fn extended_poll_closure_and_multiple_correct_answers_serialize() {
    let payload = SendPoll::new(ChatId(1), "Choose", [InputPollOption::new("A")])
        .open_period(2_628_000)
        .correct_option_ids([0])
        .allows_multiple_answers(true);
    let wire = serde_json::to_value(payload).unwrap();
    assert_eq!(wire["open_period"], 2_628_000);
    assert_eq!(wire["correct_option_ids"], json!([0]));
    assert!(wire.get("correct_option_id").is_none());
}

#[test]
fn rich_input_file_references_roundtrip_and_validate_nested_media_kind() {
    let rich = InputRichMessage::new().blocks(vec![InputRichBlockVideo::new(
        InputMediaVideo::new(InputFile::file_id(FileId("video".into()))),
    )
    .into()]);
    let wire = serde_json::to_value(&rich).unwrap();
    assert_eq!(serde_json::from_value::<InputRichMessage>(wire.clone()).unwrap(), rich);
    let mut invalid = wire;
    invalid["blocks"][0]["video"]["type"] = json!("audio");
    assert!(serde_json::from_value::<InputRichMessage>(invalid).is_err());
}

#[test]
fn game_scores_support_negative_supergroup_identifiers() {
    let payload = SetGameScore::new(UserId(1), 20, ChatId(-1001234567890), MessageId(7));
    assert_eq!(serde_json::to_value(payload).unwrap()["chat_id"], -1001234567890_i64);
}

#[test]
fn cloned_uploads_share_references_until_the_filename_changes() {
    let file = upload().file_name("original.jpg");
    let clone = file.clone();
    let original_wire = serde_json::to_value(&file).unwrap();
    assert_eq!(serde_json::to_value(&clone).unwrap(), original_wire);
    assert_ne!(serde_json::to_value(clone.file_name("renamed.jpg")).unwrap(), original_wire);
}
