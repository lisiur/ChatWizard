import { merge } from "lodash-es";
import { ThemeVars } from "../types";

const vars: ThemeVars = {
  common: {
    bodyColor: "#f3f3f3",
    baseColor: "#fff",
    textColorBase: "#000",
    textColor1: "rgb(31, 34, 37)",
    primaryColor: "#18A058",
    primaryColorHover: "#36AD6A",
    primaryColorPressed: "#0C7A43",
    primaryColorSuppl: "#36AD6A",
    infoColor: "#2080f0",
    infoColorHover: "#4098fc",
    infoColorPressed: "#1060c9",
    infoColorSuppl: "#4098fc",
    successColor: "#18a058",
    successColorHover: "#36ad6a",
    successColorPressed: "#0c7a43",
    successColorSuppl: "#36ad6a",
    warningColor: "#f0a020",
    warningColorHover: "#fcb040",
    warningColorPressed: "#c97c10",
    warningColorSuppl: "#fcb040",
    errorColor: "#d03050",
    errorColorHover: "#de576d",
    errorColorPressed: "#ab1f3f",
    errorColorSuppl: "#de576d",
    borderColor: "rgb(224,224,230)",
  },
  custom: {
    switcherBgColor: "#e0dee3",
    switcherColor: "#918f93",
    assistantMsgBgColor: "#ffffff",
    assistantMsgColor: "#000",
    userMsgBgColor: "#a9ea7a",
    userMsgColor: "#000",
    activeMenuBgColor: "#dedede",
    inputMsgColor: "#000",
    explorerBgColor: "#f7f7f7",
    explorerColor: "#000",
    explorerActiveBgColor: "#dedede",
    explorerActiveColor: "",
    chatBtnColor: "#888",
    inputBgColor: "#fff",
    codeBgColor: "#eee",
  },
};

export function override(overrides: ThemeVars) {
  merge(vars, overrides);
}

export { vars };
