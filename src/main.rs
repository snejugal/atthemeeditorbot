use attheme::{Attheme, ColorSignature};
use attheme_editor_api::{download, upload, Error, Theme, ThemeId};
use futures::future::select;
use std::{borrow::Borrow, path::Path, sync::Arc, time::Duration};
use tbot::{
    connectors,
    contexts::{traits::ChatMethods, Document, Text},
    types::{
        chat, input_file,
        keyboard::inline::{Button, ButtonKind},
    },
};
use tokio::time::delay_for;

mod localization;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.start(handle_start);

    bot.help(|context| {
        async move {
            let message = localization::help_message();
            let call_result = context.send_message(message).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.document(handle_document);

    bot.polling().start().await.unwrap();
}

async fn handle_start<C: connectors::Connector>(context: Arc<Text<C>>) {
    if context.text.value.is_empty() {
        let message = localization::start_message();
        let call_result = context.send_message(message).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    }

    let download_theme = async {
        let download = download(&context.text.value).await;
        let Theme { name, content } = match download {
            Ok(theme) => theme,
            Err(Error::BadRequest(..)) => {
                let message = localization::theme_expired();
                let call_result = context.send_message(message).call().await;

                if let Err(err) = call_result {
                    dbg!(err);
                }
                return;
            }
            Err(err) => {
                dbg!(err);
                return;
            }
        };
        let theme = content.to_bytes(ColorSignature::Hex);

        if theme.is_empty() {
            let message = localization::empty_theme();
            let call_result = context.send_message(message).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }

            return;
        }

        let file_name = format!("{}.attheme", name);
        let caption = localization::theme_file_caption();
        let document =
            input_file::Document::bytes(&file_name, &theme).caption(caption);
        let call_result = context.send_document(document).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    };

    select(Box::pin(download_theme), Box::pin(start_typing(&*context))).await;
}

async fn handle_document<C: connectors::Connector>(context: Arc<Document<C>>) {
    let wrong_file_type = async {
        let message = localization::wrong_file_type();
        let call_result = context.send_message_in_reply(message).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    };

    let file_name = if let Some(file_name) = &context.document.file_name {
        file_name
    } else {
        wrong_file_type.await;
        return;
    };

    if !file_name.ends_with(".attheme") {
        wrong_file_type.await;
        return;
    }

    let upload_theme = async {
        let call_result = context.bot.get_file(&context.document).call().await;
        let file = match call_result {
            Ok(file) => file,
            Err(err) => {
                dbg!(err);
                return;
            }
        };

        let result = context.bot.download_file(&file).await;
        let theme = match result {
            Ok(bytes) => Attheme::from_bytes(&bytes),
            Err(err) => {
                dbg!(err);
                return;
            }
        };

        let name = Path::new(&file_name).file_stem().unwrap();
        let name = name.to_string_lossy();
        let name = name.borrow();

        let theme_id = match upload(&name, &theme).await {
            Ok(ThemeId { theme_id }) => theme_id,
            Err(err) => {
                dbg!(err);
                return;
            }
        };

        let editor_link =
            format!("https://attheme-editor.snejugal.ru/?themeId={}", theme_id);
        let button_text = localization::open_theme_button(name);
        let keyboard: &[&[_]] =
            &[&[Button::new(&button_text, ButtonKind::Url(&editor_link))]];

        let message = localization::theme_uploaded();
        let call_result = context
            .send_message_in_reply(message)
            .reply_markup(keyboard)
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    };

    select(Box::pin(upload_theme), Box::pin(start_typing(&*context))).await;
}

async fn start_typing<'a, Context, Connector>(context: &Context)
where
    Context: ChatMethods<'a, Connector>,
    Connector: connectors::Connector,
{
    loop {
        let delay = delay_for(Duration::from_secs(5));
        let call_result = context
            .bot()
            .send_chat_action(context.chat().id, chat::Action::Typing)
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }

        delay.await;
    }
}
