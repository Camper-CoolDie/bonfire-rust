use super::RawItemKind;
use crate::models::publication::PostTitle;

// A title text is allowed to be 25 characters long, otherwise it's truncated
const TEXT_MAX_CHARS: usize = 25;

pub(crate) struct RawTitle {
    pub text: String,
    pub item_kind: RawItemKind,
}

impl From<RawTitle> for PostTitle {
    fn from(value: RawTitle) -> Self {
        if matches!(value.item_kind, RawItemKind::Text) && !value.text.is_empty() {
            // .chars().count() instead of .len() to account for unicode chars
            let chars_count = value.text.chars().count();

            // +3 for "..."
            let is_truncated = chars_count == TEXT_MAX_CHARS + 3;

            // When the text is truncated, the server adds "..." to the end, so we remove it
            let text = if is_truncated {
                let length = value.text.len();
                &value.text[0..length - 3]
            } else {
                &value.text
            };

            PostTitle::Text {
                text: text.to_owned(),
                is_truncated,
            }
        } else {
            PostTitle::Other {
                item_kind: value.item_kind.into(),
            }
        }
    }
}
