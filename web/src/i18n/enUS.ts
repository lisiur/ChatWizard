export const messages = {
  lang: "English",

  "common.cancel": "Cancel",
  "common.ok": "Ok",
  "common.copy": "copy",
  "common.delete": "Delete",
  "common.copy.success": "Copied to clipboard",
  "common.network.error.timeout": "Network Timeout",
  "common.network.error.connect": "Network Connect Error",
  "common.newVersion": "New version",

  "window.setting": "Setting",

  "chat.new": "New Chat",
  "chat.casual.title": "Casual Chat",
  "chat.new.defaultTitle": "New Chat",
  "chat.conversations": "Conversations",
  "chat.inputNameHint": "Please input chat name",
  "chat.delivered": "delivered",
  "chat.exportMarkdown": "Export Markdown",
  "chat.rename": "Rename",
  "chat.stick": "Pin",
  "chat.unstick": "Unpin",
  "chat.archive": "Archive",
  "chat.busy": "Please wait for the previous response to complete.",

  "chat.explorer.hidePinned": "Hide Pinned",
  "chat.explorer.showPinned": "Show Pinned",
  "chat.explorer.hideArchived": "Hide Archived",
  "chat.explorer.showArchived": "Show Archived",

  "chat.message.resend": "resend",
  "chat.message.delete": "delete",
  "chat.message.delete.hint": "Are you sure to delete this message?",
  "chat.message.stopReply": "Stop replying",

  "chat.prompt.changed": "Prompt changed to: {name}",

  "chat.config.model": "Model",
  "chat.config.model.hint": "ID of the model to use.",
  "chat.config.maxBacktrack": "Max Backtrack",
  "chat.config.maxBacktrack.hint": "Max backtrack count, 0 means no limit",
  "chat.config.temperature": "Temperature",
  "chat.config.temperature.hint":
    "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.",
  "chat.config.topP": "Top P",
  "chat.config.topP.hint":
    "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.",
  "chat.config.n": "N",
  "chat.config.n.hint":
    "How many chat completion choices to generate for each input message.",
  "chat.config.stop": "Stop",
  "chat.config.stop.hint":
    "Up to 4 sequences where the API will stop generating further tokens.",
  "chat.config.maxTokens": "Max Tokens",
  "chat.config.maxTokens.hint":
    "The maximum number of tokens to generate in the chat completion.",
  "chat.config.presencePenalty": "Presence Penalty",
  "chat.config.presencePenalty.hint":
    "Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.",
  "chat.config.frequencyPenalty": "Frequency Penalty",
  "chat.config.frequencyPenalty.hint":
    "Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.",

  "chat.export": "Export",

  "prompt.new": "New Prompt",
  "prompt.prompts": "Prompts",
  "prompt.inputNameHint": "Please input prompt name",
  "prompt.newChat": "New Chat",
  "prompt.rename": "Rename",
  "prompt.update.success": "Prompt updated successfully",

  "prompt.market.prompts": "Prompts Market",
  "prompt.market.actions.install": "Install",
  "prompt.market.actions.newChat": "New Chat",
  "prompt.market.install.success": "Prompt installed successfully",

  "plugin.market.plugins": "Plugins Market",

  "config.setting": "Setting",

  "setting.upgrade.newVersion": "New version is available",
  "setting.upgrade.cancel": "Later",
  "setting.upgrade.upgrade": "Upgrade",
  "setting.upgrade.downloading": "Downloading...",
  "setting.upgrade.relaunch": "Restart",
  "setting.upgrade.later": "Later",
  "setting.upgrade.download.success": "Download success",
  "setting.upgrade.restart.hint": "Please restart the app to apply the update.",

  "setting.locale": "Language",
  "setting.apiKey": "Api Key",
  "setting.proxy": "Proxy",
  "setting.theme": "Theme",
  "setting.theme.system": "System",
  "setting.theme.dark": "Dark",
  "setting.theme.light": "Light",
  "setting.forwardUrl": "Forward Url",
  "setting.forwardApiKey": "Forward Api Key",
  "setting.port": "Port",
  "setting.webPage": "Web Page",
  "setting.enableWebServer": "Enable Web Server",
  "setting.hideTaskbar": "Hide Taskbar",
  "setting.hideMainWindow": "Hide Main Window",
  "setting.needRestart.hint":
    "The following settings will take effect after restarting the app",
};

export default messages;

export type Messages = typeof messages;
