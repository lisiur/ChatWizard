import { merge } from "lodash-es";
import { ThemeVars } from "../types";

const vars: ThemeVars = {
  common: {
    fontSize: "1rem",
    bodyColor: "#f3f3f3",
    baseColor: "#fff",
    textColorBase: "#000",
    textColor1: "rgb(31, 34, 37)",
    primaryColor: "#18A058",
    primaryColorHover: "#36AD6A",
    primaryColorPressed: "#0C7A43",
    primaryColorSuppl: "#36AD6A",
    infoColor: "#2080F0",
    successColor: "#18A058",
    warningColor: "#F0A020",
    errorColor: "#D03050",
    borderColor: "rgb(224,224,230)",
  },
  Form: {
    labelFontSizeLeftSmall: "0.8rem",
    labelFontSizeLeftMedium: "1rem",
    labelFontSizeLeftLarge: "1.2rem",
    labelFontSizeTopMedium: "1rem",
  },
  Button: {
    fontSizeTiny: "0.7rem",
    fontSizeSmall: "0.8rem",
    fontSizeMedium: "1rem",
  },
  Tag: {
    fontSizeSmall: "0.8rem",
    fontSizeMedium: "1rem",
    heightSmall: "1.6rem",
    heightMedium: "2rem",
  },
  Input: {
    fontSizeMedium: "1rem",
  },
  Radio: {
    fontSizeMedium: "1rem",
    buttonHeightMedium: "2rem",
  },
  Switch: {
    railHeightMedium: "1.4rem",
    railWidthMedium: "2.8rem",
    buttonHeightMedium: "1rem",
    buttonWidthMedium: "1rem",
  },
  InternalSelection: {
    heightMedium: "1.8rem",
    fontSizeMedium: "1rem",
  },
  InternalSelectMenu: {
    optionFontSizeMedium: "1rem",
  },
  custom: {
    switcherBgColor: "#e0dee3",
    switcherColor: "#918f93",
    assistantMsgBgColor: "#ffffff",
    assistantMsgColor: "#000",
    codeBlockColor: "#000",
    codeBlockLangBgColor: "#ebebeb",
    codeBlockLangColor: "#000",
    userMsgBgColor: "#18A058",
    userMsgColor: "#fff",
    activeMenuBgColor: "#dedede",
    inputMsgColor: "#000",
    explorerBgColor: "#f7f7f7",
    explorerColor: "#000",
    explorerActiveBgColor: "#dedede",
    explorerActiveColor: "",
    explorerStickBgColor: "#ebebeb",
    explorerArchiveBgColor: "#ebebeb",
    chatBtnColor: "#888",
    inputBgColor: "#fff",
    codeBgColor: "#eee",
    draggingMenuBgColor: "#dedede",
    windowBorderColor: "#bbb",
    commandPanelBgColor: "#ddd",
  },
};

export function override(overrides: ThemeVars) {
  merge(vars, overrides);
}

export { vars };
