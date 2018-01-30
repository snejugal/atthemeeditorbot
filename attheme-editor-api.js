`use strict`;

const request = require(`request-promise`);

const urls = {
  upload: () => `https://snejugal.ru/attheme-editor/load-theme/`,
  download: (id) => `https://snejugal.ru/attheme-editor/get-theme/?themeId=${id}`,
  openInEditor: (id) => `https://snejugal.ru/attheme-editor/?themeId=${id}`
};

const upload = async (themeName, content) => {
  const themeId = await request({
    uri: urls.upload(),
    method: `post`,
    body: JSON.stringify({
      name: themeName,
      content: content.toString(`base64`),
    }),
  });

  return themeId;
};

const download = async (themeId) => {
  const themeData = await request(
    `https://snejugal.ru/attheme-editor/get-theme/?themeId=${themeId}`,
  );

  return JSON.parse(themeData);
};

const getOpenInEditorLink = (themeId) => urls.openInEditor(themeId);

module.exports = {
  upload,
  download,
  getOpenInEditorLink,
};