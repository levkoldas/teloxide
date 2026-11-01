use crate::types::{InputFileLike, *};

impl InputFileLike for InputMediaAnimation {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.thumbnail.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.thumbnail.move_into(into);
    }
}

impl InputFileLike for InputMediaAudio {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.thumbnail.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.thumbnail.move_into(into);
    }
}

impl InputFileLike for InputMediaDocument {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.thumbnail.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.thumbnail.move_into(into);
    }
}

impl InputFileLike for InputMediaLivePhoto {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.photo.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.photo.move_into(into);
    }
}

impl InputFileLike for InputMediaPhoto {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

impl InputFileLike for InputMediaSticker {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

impl InputFileLike for InputMediaVideo {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.thumbnail.copy_into(into);
        self.cover.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.thumbnail.move_into(into);
        self.cover.move_into(into);
    }
}

impl InputFileLike for InputMediaVoiceNote {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

impl InputFileLike for InputPaidMediaLivePhoto {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
        self.photo.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
        self.photo.move_into(into);
    }
}

impl InputFileLike for InputPollMedia {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.copy_into(into),
            Self::Audio(value) => value.copy_into(into),
            Self::Document(value) => value.copy_into(into),
            Self::LivePhoto(value) => value.copy_into(into),
            Self::Location(_) => {}
            Self::Photo(value) => value.copy_into(into),
            Self::Venue(_) => {}
            Self::Video(value) => value.copy_into(into),
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.move_into(into),
            Self::Audio(value) => value.move_into(into),
            Self::Document(value) => value.move_into(into),
            Self::LivePhoto(value) => value.move_into(into),
            Self::Location(_) => {}
            Self::Photo(value) => value.move_into(into),
            Self::Venue(_) => {}
            Self::Video(value) => value.move_into(into),
        }
    }
}

impl InputFileLike for InputPollOption {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

impl InputFileLike for InputPollOptionMedia {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.copy_into(into),
            Self::Link(_) => {}
            Self::LivePhoto(value) => value.copy_into(into),
            Self::Location(_) => {}
            Self::Photo(value) => value.copy_into(into),
            Self::Sticker(value) => value.copy_into(into),
            Self::Venue(_) => {}
            Self::Video(value) => value.copy_into(into),
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.move_into(into),
            Self::Link(_) => {}
            Self::LivePhoto(value) => value.move_into(into),
            Self::Location(_) => {}
            Self::Photo(value) => value.move_into(into),
            Self::Sticker(value) => value.move_into(into),
            Self::Venue(_) => {}
            Self::Video(value) => value.move_into(into),
        }
    }
}

impl InputFileLike for InputRichBlock {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Paragraph(_) => {}
            Self::SectionHeading(_) => {}
            Self::Preformatted(_) => {}
            Self::Footer(_) => {}
            Self::Divider(_) => {}
            Self::MathematicalExpression(_) => {}
            Self::Anchor(_) => {}
            Self::List(value) => value.copy_into(into),
            Self::BlockQuotation(value) => value.copy_into(into),
            Self::ExpandableBlockQuotation(_) => {}
            Self::PullQuotation(_) => {}
            Self::Collage(value) => value.copy_into(into),
            Self::Slideshow(value) => value.copy_into(into),
            Self::Table(_) => {}
            Self::Details(value) => value.copy_into(into),
            Self::Map(_) => {}
            Self::Buttons(_) => {}
            Self::Animation(value) => value.copy_into(into),
            Self::Audio(value) => value.copy_into(into),
            Self::Document(value) => value.copy_into(into),
            Self::Photo(value) => value.copy_into(into),
            Self::Video(value) => value.copy_into(into),
            Self::VoiceNote(value) => value.copy_into(into),
            Self::Thinking(_) => {}
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Paragraph(_) => {}
            Self::SectionHeading(_) => {}
            Self::Preformatted(_) => {}
            Self::Footer(_) => {}
            Self::Divider(_) => {}
            Self::MathematicalExpression(_) => {}
            Self::Anchor(_) => {}
            Self::List(value) => value.move_into(into),
            Self::BlockQuotation(value) => value.move_into(into),
            Self::ExpandableBlockQuotation(_) => {}
            Self::PullQuotation(_) => {}
            Self::Collage(value) => value.move_into(into),
            Self::Slideshow(value) => value.move_into(into),
            Self::Table(_) => {}
            Self::Details(value) => value.move_into(into),
            Self::Map(_) => {}
            Self::Buttons(_) => {}
            Self::Animation(value) => value.move_into(into),
            Self::Audio(value) => value.move_into(into),
            Self::Document(value) => value.move_into(into),
            Self::Photo(value) => value.move_into(into),
            Self::Video(value) => value.move_into(into),
            Self::VoiceNote(value) => value.move_into(into),
            Self::Thinking(_) => {}
        }
    }
}

impl InputFileLike for InputRichBlockAnimation {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.animation.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.animation.move_into(into);
    }
}

impl InputFileLike for InputRichBlockAudio {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.audio.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.audio.move_into(into);
    }
}

impl InputFileLike for InputRichBlockBlockQuotation {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

impl InputFileLike for InputRichBlockCollage {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

impl InputFileLike for InputRichBlockDetails {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

impl InputFileLike for InputRichBlockDocument {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.document.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.document.move_into(into);
    }
}

impl InputFileLike for InputRichBlockList {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.items.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.items.move_into(into);
    }
}

impl InputFileLike for InputRichBlockListItem {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

impl InputFileLike for InputRichBlockPhoto {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.photo.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.photo.move_into(into);
    }
}

impl InputFileLike for InputRichBlockSlideshow {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

impl InputFileLike for InputRichBlockVideo {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.video.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.video.move_into(into);
    }
}

impl InputFileLike for InputRichBlockVoiceNote {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.voice_note.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.voice_note.move_into(into);
    }
}

impl InputFileLike for InputRichMedia {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.copy_into(into),
            Self::Audio(value) => value.copy_into(into),
            Self::Document(value) => value.copy_into(into),
            Self::Photo(value) => value.copy_into(into),
            Self::Video(value) => value.copy_into(into),
            Self::VoiceNote(value) => value.copy_into(into),
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation(value) => value.move_into(into),
            Self::Audio(value) => value.move_into(into),
            Self::Document(value) => value.move_into(into),
            Self::Photo(value) => value.move_into(into),
            Self::Video(value) => value.move_into(into),
            Self::VoiceNote(value) => value.move_into(into),
        }
    }
}

impl InputFileLike for InputRichMessage {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
        self.media.move_into(into);
    }
}

impl InputFileLike for InputRichMessageContent {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.rich_message.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.rich_message.move_into(into);
    }
}

impl InputFileLike for InputRichMessageMedia {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

impl InputFileLike for InputMessageContent {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        if let Self::RichMessage(content) = self {
            content.copy_into(into);
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Self::RichMessage(content) = self {
            content.move_into(into);
        }
    }
}
impl InputFileLike for InlineQueryResult {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::CachedAudio(value) => value.input_message_content.copy_into(into),
            Self::CachedDocument(value) => value.input_message_content.copy_into(into),
            Self::CachedGif(value) => value.input_message_content.copy_into(into),
            Self::CachedMpeg4Gif(value) => value.input_message_content.copy_into(into),
            Self::CachedPhoto(value) => value.input_message_content.copy_into(into),
            Self::CachedSticker(value) => value.input_message_content.copy_into(into),
            Self::CachedVideo(value) => value.input_message_content.copy_into(into),
            Self::CachedVoice(value) => value.input_message_content.copy_into(into),
            Self::Article(value) => value.input_message_content.copy_into(into),
            Self::Audio(value) => value.input_message_content.copy_into(into),
            Self::Contact(value) => value.input_message_content.copy_into(into),
            Self::Document(value) => value.input_message_content.copy_into(into),
            Self::Gif(value) => value.input_message_content.copy_into(into),
            Self::Location(value) => value.input_message_content.copy_into(into),
            Self::Mpeg4Gif(value) => value.input_message_content.copy_into(into),
            Self::Photo(value) => value.input_message_content.copy_into(into),
            Self::Venue(value) => value.input_message_content.copy_into(into),
            Self::Video(value) => value.input_message_content.copy_into(into),
            Self::Voice(value) => value.input_message_content.copy_into(into),
            Self::Game(_) => {}
        }
    }
    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::CachedAudio(value) => value.input_message_content.move_into(into),
            Self::CachedDocument(value) => value.input_message_content.move_into(into),
            Self::CachedGif(value) => value.input_message_content.move_into(into),
            Self::CachedMpeg4Gif(value) => value.input_message_content.move_into(into),
            Self::CachedPhoto(value) => value.input_message_content.move_into(into),
            Self::CachedSticker(value) => value.input_message_content.move_into(into),
            Self::CachedVideo(value) => value.input_message_content.move_into(into),
            Self::CachedVoice(value) => value.input_message_content.move_into(into),
            Self::Article(value) => value.input_message_content.move_into(into),
            Self::Audio(value) => value.input_message_content.move_into(into),
            Self::Contact(value) => value.input_message_content.move_into(into),
            Self::Document(value) => value.input_message_content.move_into(into),
            Self::Gif(value) => value.input_message_content.move_into(into),
            Self::Location(value) => value.input_message_content.move_into(into),
            Self::Mpeg4Gif(value) => value.input_message_content.move_into(into),
            Self::Photo(value) => value.input_message_content.move_into(into),
            Self::Venue(value) => value.input_message_content.move_into(into),
            Self::Video(value) => value.input_message_content.move_into(into),
            Self::Voice(value) => value.input_message_content.move_into(into),
            Self::Game(_) => {}
        }
    }
}
