use lazy_static::lazy_static;
use tbot::markup::{bold, inline_code, link, markdown_v2};
use tbot::types::parameters::Text;

pub fn start_message() -> Text<'static> {
    lazy_static! {
        static ref START_MESSAGE: String = markdown_v2((
            "Hello! I'm a bot that makes it easier to open themes in ",
            link(".attheme editor", "https://attheme-editor.snejugal.ru"),
            ". Just send me an ",
            inline_code([".attheme"]),
            " file, and I will send you the link to open it.",
        ))
        .to_string();
    }

    Text::markdown_v2(&START_MESSAGE)
}

pub fn help_message() -> Text<'static> {
    lazy_static! {
        static ref HELP_MESSAGE: String = markdown_v2((
            "I'm a bot that makes it easier to open themes in ",
            link(".attheme editor", "https://attheme-editor.snejugal.ru"),
            ". Just send me an ",
            inline_code([".attheme"]),
            " file, and I will send you the link to open it.",
        ))
        .to_string();
    }

    Text::markdown_v2(&HELP_MESSAGE)
}

pub fn wrong_file_type() -> Text<'static> {
    lazy_static! {
        static ref WRONG_FILE_TYPE: String = markdown_v2((
            "Hmm, looks like you sent a wrong file — \
             I only know how to work with ",
            inline_code([".attheme"]),
            " files.",
        ))
        .to_string();
    }

    Text::markdown_v2(&WRONG_FILE_TYPE)
}

pub fn open_theme_button(theme: &str) -> String {
    format!("Open {} in the editor", theme)
}

pub fn theme_uploaded() -> Text<'static> {
    lazy_static! {
        static ref THEME_UPLOADED: String = markdown_v2((
            "The theme is ready to be opened in the editor! \
             Just tap the button below.\n\n",
            bold("Warning:"),
            " Once you use the link, it expires and can't be used anymore.",
        ))
        .to_string();
    }

    Text::markdown_v2(&THEME_UPLOADED)
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
