import { merge } from "lodash-es";
import { ThemeVars } from "../types";

const vars: ThemeVars = {
  common: {
    bodyColor: "#111111",
    baseColor: "#000",
    textColorBase: "#fff",
    textColor1: "rgba(255, 255, 255, 0.9)",
    primaryColor: "#63E2B7",
    primaryColorHover: "#7FE7C4",
    primaryColorPressed: "#5ACEA7",
    primaryColorSuppl: "#2A947D",
    infoColor: "#70C0E8",
    infoColorHover: "#8ACBEC",
    infoColorPressed: "#66AFD3",
    infoColorSuppl: "rgb(56, 137, 197)",
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
    borderColor: "#333",
  },
  custom: {
    assistantMsgBgColor: "#2c2c2c",
    assistantMsgColor: "#fff",
    userMsgBgColor: "#59b269",
    userMsgColor: "#000",
    activeMenuBgColor: "#363636",
    inputMsgColor: "#fff",
    explorerBgColor: "#191919",
    explorerColor: "#fff",
    explorerActiveBgColor: "#363636",
    explorerActiveColor: "",
    chatBtnColor: "#666",
  },
};

export function override(overrides: ThemeVars) {
  merge(vars, overrides);
}

export default vars;
