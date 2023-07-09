import { GlobalThemeOverrides } from "naive-ui";

export interface ThemeVars extends GlobalThemeOverrides {
  custom?: {
    switcherBgColor: string;
    switcherColor: string;
    assistantMsgBgColor: string;
    assistantMsgColor: string;
    codeBlockColor: string;
    codeBlockLangBgColor: string;
    codeBlockLangColor: string;
    userMsgBgColor: string;
    userMsgColor: string;
    activeMenuBgColor: string;
    explorerBgColor: string;
    inputMsgColor: string;
    explorerColor: string;
    explorerActiveBgColor: string;
    explorerActiveColor: string;
    explorerStickBgColor: string;
    explorerArchiveBgColor: string;
    chatBtnColor: string;
    inputBgColor: string;
    codeBgColor: string;
    draggingMenuBgColor: string;
    windowBorderColor: string,
    commandPanelBgColor: string,
  };
}
