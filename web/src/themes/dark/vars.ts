import { merge } from "lodash-es";
import { ThemeVars } from "../types";

const vars: ThemeVars = {
  common: {
    fontSize: "1rem",
    bodyColor: "#111111",
    baseColor: "#000",
    textColorBase: "#fff",
    textColor1: "rgba(255, 255, 255, 0.9)",
    primaryColor: "#59b269",
    primaryColorHover: "#8cc78c",
    primaryColorPressed: "#548454",
    primaryColorSuppl: "#59b269",
    infoColor: "#70C0E8",
    successColor: "#63E2B7",
    warningColor: "#F2C97D",
    errorColor: "#E88080",
    borderColor: "#333",
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
    switcherBgColor: "#222027",
    switcherColor: "#918f93",
    assistantMsgBgColor: "#2c2c2c",
    assistantMsgColor: "#fff",
    codeBlockColor: "#fff",
    codeBlockLangBgColor: "#222",
    codeBlockLangColor: "#ccc",
    userMsgBgColor: "#59b269",
    userMsgColor: "#000",
    activeMenuBgColor: "#363636",
    inputMsgColor: "#fff",
    explorerBgColor: "#191919",
    explorerColor: "#fff",
    explorerActiveBgColor: "#363636",
    explorerActiveColor: "",
    explorerStickBgColor: "#202020",
    explorerArchiveBgColor: "#202020",
    chatBtnColor: "#666",
    inputBgColor: "#2c2c2c",
    codeBgColor: "#111111",
    draggingMenuBgColor: "#363636",
    windowBorderColor: "#444",
    commandPanelBgColor: "#333",
  },
};

export function override(overrides: ThemeVars) {
  merge(vars, overrides);
}

export default vars;
