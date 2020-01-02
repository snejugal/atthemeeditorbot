use tbot::types::parameters::Text;

pub fn start_message() -> Text<'static> {
    Text::markdown_v2(
        "Hello\\! I'm a bot that makes it easier to open themes in \
         [\\.attheme editor](https://snejugal.ru/attheme-editor)\\. Just send \
         me an \\.attheme file, and I will send you the link to open it\\.",
    )
}

pub fn help_message() -> Text<'static> {
    Text::markdown_v2(
        "I'm a bot that makes it easier to open themes in \
         [\\.attheme editor](https://snejugal.ru/attheme-editor)\\. Just send \
         me an \\.attheme file, and I will send you the link to open it\\.",
    )
}

pub fn wrong_file_type() -> Text<'static> {
    Text::plain(
        "Hmm, looks like you sent a wrong file — \
         I only know how to work with .attheme files.",
    )
}

pub fn open_theme_button(theme: &str) -> String {
    format!("Open {} in the editor", theme)
}

pub fn theme_uploaded() -> Text<'static> {
    Text::markdown_v2(
        "The theme is ready to be opened in the editor\\! \
        Just tap the button below\\.\n\n\

        *Warning:* Once you use the link, it expires and can't be used \
        anymore\\.",
    )
}

pub fn empty_theme() -> Text<'static> {
    Text::plain("Oops, your theme is empty, Telegram doesn't allow such ones.")
}

pub fn theme_file_caption() -> Text<'static> {
    Text::plain("Here you go!")
}

pub fn theme_expired() -> Text<'static> {
    Text::plain("Hmm. Looks like the theme has already expired.")
}
