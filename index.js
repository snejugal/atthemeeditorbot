`use strict`;

const Telegraf = require(`telegraf`);
const request = require(`request-promise`);

const { token, options } = require(`./config.json`);
const localization = require(`./localization`);
const atthemeEditor = require(`./attheme-editor-api`);

const bot = new Telegraf(token, options);

bot.context.sendChatAction = function (action) {
  bot.telegram.sendChatAction(this.chat.id, action);
};

bot.context.startSendingChatAction = function (action = `typing`) {
  const interval = setInterval(() => this.sendChatAction(action), 4000);
  this.sendChatAction(action);

  const stop = () => clearInterval(interval);

  return {
    stop,
  };
};

bot.context.downloadFile = async function () {
  const file = await bot.telegram.getFile(this.document.file_id);

  const result = await request({
    uri: `http://api.telegram.org/file/bot${token}/${file.file_path}`,
    encoding: null,
  });

  return result;
};

bot.context.createInlineKeyboard = (...rows) => {
  return {
    reply_markup: {
      inline_keyboard: rows, // rows are already cooked ¯\_(ツ)_/¯
    },
  };
};

bot.context.MARKDOWN = {
  parse_mode: `Markdown`,
};

bot.use((context, next) => {
  if (`document` in context.message) {
    context.document = context.message.document;
  }

  next();
});

bot.command(`start`, async (context) => {
  const themeId = context.message.text.slice(`/start `.length);

  if (themeId) {
    const typing = context.startSendingChatAction();

    try {
      const { name, content } = await atthemeEditor.download(themeId);

      if (content.length == 0) {
        context.reply(localization.en.themeIsEmpty());
      } else {
        context.replyWithDocument(
          {
            source: Buffer.from(content, `base64`),
            filename: `${name}.attheme`,
          },
          {
            caption: localization.en.themeFileCaption(),
          },
        );
      }
    } catch (error) {
      context.reply(localization.en.themeAlreadyDownloaded());
    } finally {
      typing.stop();
    }
  } else {
    context.reply(
      localization.en.startMessage(),
      context.MARKDOWN,
    );
  }
});

bot.command(`help`, (context) => {
  context.reply(
    localization.en.helpMessage(),
    context.MARKDOWN,
  );
});

bot.on(`document`, async (context) => {
  const { document } = context;

  if (document.file_name.endsWith(`.attheme`)) {
    const typing = context.startSendingChatAction();

    const themeContent = await context.downloadFile();
    const themeName = document.file_name.slice(0, -`.attheme`.length);
    const themeId = await atthemeEditor.upload(themeName, themeContent);

    const openEditorLink = atthemeEditor.getOpenInEditorLink(themeId);
    const openEditorButtonText = localization.en.openThemeButton(themeName);

    context.reply(
      localization.en.themeUploaded(),
      {
        ...context.MARKDOWN,
        ...context.createInlineKeyboard(
          [
            {
              text: openEditorButtonText,
              url: openEditorLink,
            },
          ],
        ),
      },
    );

    typing.stop();
  } else {
    context.reply(localization.en.wrongFileType());
  }
});

bot.startPolling();
console.log(`@atthemeeditorbot is running…`);