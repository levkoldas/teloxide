use crate::payloads::{
    AnswerGuestQuery, EditEphemeralMessageCaption, EditEphemeralMessageMedia,
    EditEphemeralMessageText, SendLivePhoto, SendMessageDraft, SendRichMessage,
    SendRichMessageDraft,
};
use std::future::IntoFuture;

use url::Url;

use crate::{
    payloads::{
        AnswerInlineQuery, AnswerWebAppQuery, CopyMessage, EditMessageCaption,
        EditMessageCaptionInline, EditMessageChecklist, EditMessageMedia, EditMessageMediaInline,
        EditMessageText, EditMessageTextInline, EditStory, GiftPremiumSubscription, PostStory,
        SavePreparedInlineMessage, SendAnimation, SendAudio, SendChecklist, SendDocument, SendGift,
        SendGiftChat, SendMediaGroup, SendMessage, SendPaidMedia, SendPhoto, SendPoll, SendVideo,
        SendVoice,
    },
    prelude::Requester,
    requests::{HasPayload, Output, Request},
    types::*,
};

/// Default parse mode adaptor, see
/// [`RequesterExt::parse_mode`](crate::requests::RequesterExt::parse_mode).
#[derive(Clone, Debug)]
pub struct DefaultParseMode<B> {
    bot: B,
    mode: ParseMode,
}

/// Request returned by [`DefaultParseMode`] methods.
#[derive(Clone)]
pub struct DefaultParseModeRequest<R> {
    req: R,
    mode: ParseMode,
}

impl<B> DefaultParseMode<B> {
    /// Creates new [`DefaultParseMode`].
    ///
    /// Note: it's recommended to use [`RequesterExt::parse_mode`] instead.
    ///
    /// [`RequesterExt::parse_mode`]: crate::requests::RequesterExt::parse_mode
    pub fn new(bot: B, parse_mode: ParseMode) -> Self {
        Self { bot, mode: parse_mode }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Returns currently used [`ParseMode`].
    pub fn parse_mode(&self) -> ParseMode {
        self.mode
    }
}

impl<R> Request for DefaultParseModeRequest<R>
where
    R: Request + Clone,
    R::Payload: VisitParseModes,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::Send;

    // Required methods
    fn send(mut self) -> Self::Send {
        self.req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(self.mode));
        self.req.send()
    }

    fn send_ref(&self) -> Self::SendRef {
        // There is no other way to change the payload, given a `&self` :(
        self.clone().send()
    }
}

impl<R> IntoFuture for DefaultParseModeRequest<R>
where
    Self: Request,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R> HasPayload for DefaultParseModeRequest<R>
where
    R: Request,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.req.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.req.payload_ref()
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        {
            let req = $this.inner().$m($($arg),*);
            DefaultParseModeRequest { req, mode: $this.mode }
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        DefaultParseModeRequest<B::$T>
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

impl<B> Requester for DefaultParseMode<B>
where
    B: Requester,
    B::SendMessage: Clone,
    B::SendLivePhoto: Clone,
    B::SendMessageDraft: Clone,
    B::AnswerGuestQuery: Clone,
    B::EditEphemeralMessageMedia: Clone,
    B::EditEphemeralMessageCaption: Clone,
    B::EditEphemeralMessageText: Clone,
    B::SendRichMessage: Clone,
    B::SendRichMessageDraft: Clone,
    B::SendPhoto: Clone,
    B::SendVideo: Clone,
    B::SendAudio: Clone,
    B::SendDocument: Clone,
    B::SendAnimation: Clone,
    B::SendVoice: Clone,
    B::EditMessageText: Clone,
    B::EditMessageTextInline: Clone,
    B::EditMessageCaption: Clone,
    B::EditMessageCaptionInline: Clone,
    B::SendPoll: Clone,
    B::SendChecklist: Clone,
    B::CopyMessage: Clone,
    B::PostStory: Clone,
    B::EditStory: Clone,
    B::AnswerInlineQuery: Clone,
    B::AnswerWebAppQuery: Clone,
    B::SavePreparedInlineMessage: Clone,
    B::EditMessageMedia: Clone,
    B::EditMessageMediaInline: Clone,
    B::EditMessageChecklist: Clone,
    B::SendPaidMedia: Clone,
    B::SendMediaGroup: Clone,
    B::GiftPremiumSubscription: Clone,
    B::SendGift: Clone,
    B::SendGiftChat: Clone,
{
    type Err = B::Err;

    requester_forward! {
        edit_ephemeral_message_text,
        send_rich_message,
        send_rich_message_draft,

        send_live_photo,
        send_message_draft,
        answer_guest_query,
        edit_ephemeral_message_media,
        edit_ephemeral_message_caption,

        send_message,
        send_photo,
        send_video,
        send_audio,
        send_document,
        send_animation,
        send_voice,
        send_poll,
        send_checklist,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_checklist,
        copy_message,
        post_story,
        edit_story,
        answer_inline_query,
        answer_web_app_query,
        save_prepared_inline_message,
        send_paid_media,
        send_media_group,
        edit_message_media,
        edit_message_media_inline,
        gift_premium_subscription,
        send_gift,
        send_gift_chat,
        => f, fty
    }

    requester_forward! {
        get_user_profile_audios,
        set_chat_member_tag,
        answer_chat_join_request_query,
        send_chat_join_request_web_app,
        get_user_personal_chat_messages,
        get_managed_bot_token,
        replace_managed_bot_token,
        get_managed_bot_access_settings,
        set_managed_bot_access_settings,
        set_my_profile_photo,
        remove_my_profile_photo,
        get_user_gifts,
        get_chat_gifts,
        repost_story,
        save_prepared_keyboard_button,
        edit_ephemeral_message_reply_markup,
        delete_ephemeral_message,
        delete_message_reaction,
        delete_all_message_reactions,

        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_messages,
        send_video_note,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_dice,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        set_user_emoji_status,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        create_chat_subscription_invite_link,
        edit_chat_subscription_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        approve_suggested_post,
        decline_suggested_post,
        delete_message,
        delete_messages,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        get_available_gifts,
        verify_user,
        verify_chat,
        remove_user_verification,
        remove_chat_verification,
        read_business_message,
        delete_business_messages,
        set_business_account_name,
        set_business_account_username,
        set_business_account_bio,
        set_business_account_profile_photo,
        remove_business_account_profile_photo,
        set_business_account_gift_settings,
        get_business_account_star_balance,
        transfer_business_account_stars,
        get_business_account_gifts,
        convert_gift_to_stars,
        upgrade_gift,
        transfer_gift,
        delete_story,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_my_star_balance,
        get_star_transactions,
        refund_star_payment,
        edit_user_star_subscription,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fid, ftyid
    }
}

download_forward! {
    B
    DefaultParseMode<B>
    { this => this.inner() }
}

trait VisitParseModes {
    fn visit_parse_modes(&mut self, visitor: impl FnMut(&mut Option<ParseMode>));
}

macro_rules! impl_visit_parse_modes {
    (
        $(
            $T:ty => [
                $(
                    $field:ident
                ),*
            ]
            ,
        )*
    ) => {
        $(
            impl VisitParseModes for $T {
                fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
                    $(
                        visitor(&mut self.$field);
                    )*
                }
            }
        )*
    }
}

impl_visit_parse_modes! {
    SendMessage => [parse_mode],
    SendLivePhoto => [parse_mode],
    SendMessageDraft => [parse_mode],
    EditEphemeralMessageCaption => [parse_mode],
    SendPhoto => [parse_mode],
    SendVideo => [parse_mode],
    SendAudio => [parse_mode],
    SendDocument => [parse_mode],
    SendAnimation => [parse_mode],
    SendVoice => [parse_mode],


    EditMessageCaption => [parse_mode],
    EditMessageCaptionInline => [parse_mode],
    SendPaidMedia => [parse_mode],
    GiftPremiumSubscription => [text_parse_mode],
    SendGift => [text_parse_mode],
    SendGiftChat => [text_parse_mode],
    // FIXME: check if `parse_mode` changes anything if `.caption` is not set
    //        (and if it does, maybe not call visitor if `self.caption.is_none()`)
    CopyMessage => [parse_mode],
    PostStory => [parse_mode],
    EditStory => [parse_mode],

}

impl VisitParseModes for AnswerInlineQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.results
            .iter_mut()
            .for_each(|result| visit_parse_modes_in_inline_query_result(result, &mut visitor))
    }
}

impl VisitParseModes for AnswerWebAppQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SavePreparedInlineMessage {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SendMediaGroup {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.media
            .iter_mut()
            .for_each(|media| visit_parse_modes_in_input_media(media, &mut visitor))
    }
}

impl VisitParseModes for EditMessageMedia {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageMediaInline {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

impl VisitParseModes for SendChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

fn visit_parse_modes_in_inline_query_result(
    result: &mut InlineQueryResult,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    result.visit_nested(visitor);
}

fn visit_parse_modes_in_input_media(
    media: &mut InputMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputMedia::*;

    let parse_mode = match media {
        LivePhoto(m) => &mut m.parse_mode,
        Photo(m) => &mut m.parse_mode,
        Video(m) => &mut m.parse_mode,
        Animation(m) => &mut m.parse_mode,
        Audio(m) => &mut m.parse_mode,
        Document(m) => &mut m.parse_mode,
    };

    visitor(parse_mode);
}

impl VisitParseModes for AnswerGuestQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for EditEphemeralMessageMedia {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

// Recursion uses a borrowed visitor so its type does not grow with every block.
trait VisitNestedParseModes {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>));
}
impl<T: VisitNestedParseModes> VisitNestedParseModes for Option<T> {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        if let Some(value) = self {
            value.visit_nested(visitor);
        }
    }
}
impl<T: VisitNestedParseModes> VisitNestedParseModes for Vec<T> {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        for value in self {
            value.visit_nested(visitor);
        }
    }
}
impl<T: VisitNestedParseModes> VisitNestedParseModes for Box<T> {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        (**self).visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputMedia {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Animation(value) => value.visit_nested(visitor),
            Self::Audio(value) => value.visit_nested(visitor),
            Self::Document(value) => value.visit_nested(visitor),
            Self::LivePhoto(value) => value.visit_nested(visitor),
            Self::Photo(value) => value.visit_nested(visitor),
            Self::Video(value) => value.visit_nested(visitor),
        }
    }
}
impl VisitNestedParseModes for crate::types::InputMediaAnimation {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaAudio {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaDocument {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaLivePhoto {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaPhoto {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaVideo {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputMediaVoiceNote {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
    }
}
impl VisitNestedParseModes for crate::types::InputPollMedia {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Animation(value) => value.visit_nested(visitor),
            Self::Audio(value) => value.visit_nested(visitor),
            Self::Document(value) => value.visit_nested(visitor),
            Self::LivePhoto(value) => value.visit_nested(visitor),
            Self::Location(_) => {}
            Self::Photo(value) => value.visit_nested(visitor),
            Self::Venue(_) => {}
            Self::Video(value) => value.visit_nested(visitor),
        }
    }
}
impl VisitNestedParseModes for crate::types::InputPollOptionMedia {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Animation(value) => value.visit_nested(visitor),
            Self::Link(_) => {}
            Self::LivePhoto(value) => value.visit_nested(visitor),
            Self::Location(_) => {}
            Self::Photo(value) => value.visit_nested(visitor),
            Self::Sticker(_) => {}
            Self::Venue(_) => {}
            Self::Video(value) => value.visit_nested(visitor),
        }
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlock {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Paragraph(_) => {}
            Self::SectionHeading(_) => {}
            Self::Preformatted(_) => {}
            Self::Footer(_) => {}
            Self::Divider(_) => {}
            Self::MathematicalExpression(_) => {}
            Self::Anchor(_) => {}
            Self::List(value) => value.visit_nested(visitor),
            Self::BlockQuotation(value) => value.visit_nested(visitor),
            Self::ExpandableBlockQuotation(_) => {}
            Self::PullQuotation(_) => {}
            Self::Collage(value) => value.visit_nested(visitor),
            Self::Slideshow(value) => value.visit_nested(visitor),
            Self::Table(_) => {}
            Self::Details(value) => value.visit_nested(visitor),
            Self::Map(_) => {}
            Self::Buttons(_) => {}
            Self::Animation(value) => value.visit_nested(visitor),
            Self::Audio(value) => value.visit_nested(visitor),
            Self::Document(value) => value.visit_nested(visitor),
            Self::Photo(value) => value.visit_nested(visitor),
            Self::Video(value) => value.visit_nested(visitor),
            Self::VoiceNote(value) => value.visit_nested(visitor),
            Self::Thinking(_) => {}
        }
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockAnimation {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.animation.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockAudio {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.audio.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockBlockQuotation {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockCollage {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockDetails {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockDocument {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.document.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockList {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.items.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockListItem {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockPhoto {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.photo.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockSlideshow {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockVideo {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.video.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichBlockVoiceNote {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.voice_note.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichMedia {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Animation(value) => value.visit_nested(visitor),
            Self::Audio(value) => value.visit_nested(visitor),
            Self::Document(value) => value.visit_nested(visitor),
            Self::Photo(value) => value.visit_nested(visitor),
            Self::Video(value) => value.visit_nested(visitor),
            Self::VoiceNote(value) => value.visit_nested(visitor),
        }
    }
}
impl VisitNestedParseModes for crate::types::InputRichMessage {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.blocks.visit_nested(visitor);
        self.media.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichMessageContent {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.rich_message.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for crate::types::InputRichMessageMedia {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        self.media.visit_nested(visitor);
    }
}

impl VisitNestedParseModes for crate::types::InputMessageContent {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::Text(content) => visitor(&mut content.parse_mode),
            Self::RichMessage(content) => content.visit_nested(visitor),
            _ => {}
        }
    }
}
impl VisitNestedParseModes for crate::types::InputPollOption {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        use crate::types::InputPollOptionFormatting;
        if self.formatting.is_none() {
            let mut mode = None;
            visitor(&mut mode);
            self.formatting = mode.map(InputPollOptionFormatting::TextParseMode);
        }
        self.media.visit_nested(visitor);
    }
}
impl VisitNestedParseModes for InlineQueryResult {
    fn visit_nested(&mut self, visitor: &mut dyn FnMut(&mut Option<ParseMode>)) {
        match self {
            Self::CachedAudio(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedDocument(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedGif(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedMpeg4Gif(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedPhoto(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedSticker(value) => {
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedVideo(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::CachedVoice(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Article(value) => {
                value.input_message_content.visit_nested(visitor);
            }
            Self::Audio(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Contact(value) => {
                value.input_message_content.visit_nested(visitor);
            }
            Self::Game(_) => {}
            Self::Document(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Gif(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Location(value) => {
                value.input_message_content.visit_nested(visitor);
            }
            Self::Mpeg4Gif(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Photo(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Venue(value) => {
                value.input_message_content.visit_nested(visitor);
            }
            Self::Video(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
            Self::Voice(value) => {
                visitor(&mut value.parse_mode);
                value.input_message_content.visit_nested(visitor);
            }
        }
    }
}
impl VisitParseModes for EditMessageText {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
        self.rich_message.visit_nested(&mut visitor);
    }
}
impl VisitParseModes for EditMessageTextInline {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.parse_mode);
        self.rich_message.visit_nested(&mut visitor);
    }
}
impl VisitParseModes for SendPoll {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.question_parse_mode);
        visitor(&mut self.explanation_parse_mode);
        visitor(&mut self.description_parse_mode);
        self.options.visit_nested(&mut visitor);
        self.media.visit_nested(&mut visitor);
        self.explanation_media.visit_nested(&mut visitor);
    }
}
impl VisitParseModes for SendRichMessage {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.rich_message.visit_nested(&mut visitor);
    }
}
impl VisitParseModes for SendRichMessageDraft {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.rich_message.visit_nested(&mut visitor);
    }
}
impl VisitParseModes for EditEphemeralMessageText {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.rich_message.visit_nested(&mut visitor);
        visitor(&mut self.parse_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_parse_mode_reaches_nested_rich_and_poll_media() {
        let photo = || {
            InputMediaPhoto::new(InputFile::file_id(FileId("photo".into())))
                .caption("<b>caption</b>")
        };
        let rich = InputRichMessage::new().blocks(vec![InputRichBlockDetails::new(
            "Details".into(),
            vec![InputRichBlockPhoto::new(photo()).into()],
        )
        .into()]);
        let mut rich_payload = SendRichMessage::new(ChatId(1), rich);
        rich_payload.visit_parse_modes(|mode| {
            mode.get_or_insert(ParseMode::Html);
        });
        let encoded = serde_json::to_value(rich_payload).unwrap();
        assert_eq!(
            encoded["rich_message"]["blocks"][0]["blocks"][0]["photo"]["parse_mode"],
            "HTML"
        );

        let mut poll = SendPoll::new(
            ChatId(1),
            "Question",
            [InputPollOption::new("Option").text_entities(vec![]).media(photo().into())],
        );
        poll.visit_parse_modes(|mode| {
            mode.get_or_insert(ParseMode::Html);
        });
        let encoded = serde_json::to_value(poll).unwrap();
        assert_eq!(encoded["question_parse_mode"], "HTML");
        assert_eq!(encoded["options"][0]["text_entities"], json!([]));
        assert!(encoded["options"][0].get("text_parse_mode").is_none());
        assert_eq!(encoded["options"][0]["media"]["parse_mode"], "HTML");
    }
}
