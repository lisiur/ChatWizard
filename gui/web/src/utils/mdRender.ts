import { marked } from "marked";
import hljs from "highlight.js";
import { onThemeChanged, getTheme } from "../themes";

marked.setOptions({
  highlight(code, lang) {
    const language = hljs.getLanguage(lang) ? lang : "plaintext";
    return hljs.highlight(code, { language }).value;
  },
});

reloadTheme();
onThemeChanged(() => {
  reloadTheme();
});

function reloadTheme() {
  const theme = getTheme();
  if (theme === "light") {
    import("highlight.js/styles/github.css");
  } else {
    import("highlight.js/styles/github-dark-dimmed.css");
  }
}

export default marked.parse;
