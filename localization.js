`use strict`;

const en = {
  startMessage:
    () => `Hello! I'm a bot that makes it easier to open themes in [.attheme editor](https://snejugal.ru/attheme-editor). Just send me an .attheme file, and I will send you the link to open it.`,
  themeFileCaption:
    () => `Here you go!`,
  themeIsEmpty:
    () => `Oops, your theme is empty, Telegram doesn't allow such ones.`,
  themeAlreadyDownloaded:
    () => `Hmm. Looks like the theme has already been downloaded.`,
  helpMessage:
    () => `I'm a bot that makes it easier to open themes in [.attheme editor](https://snejugal.ru/attheme-editor). Just send me an .attheme file, and I will send you the link to open it.`,
  themeUploaded:
    () => `The theme is ready to be opened in the editor! Just tap the button below.

*Warning:* Once you use the link, it expires and can't be used anymore.`,
  openThemeButton:
    (themeName) => `Open ${themeName} in the editor`,
  wrongFileType:
    () => `Hmm, looks like you sent a wrong file — I only know how to work with .attheme files.`,
};

// TODO: add support for other languages

module.exports = {
  en,
};