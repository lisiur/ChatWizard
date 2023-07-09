import { marked } from "marked";
import hljs from "highlight.js";
import { onThemeChanged, getTheme } from "../themes";
import { i18n } from "../hooks/i18n";
import { writeToClipboard } from "./api";
import { message } from "./prompt";

const { t } = i18n.global;

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

const renderer = new marked.Renderer();
renderer.code = (code, lang) => {
  lang ??= "plaintext";
  const language = hljs.getLanguage(lang) ? lang : "plaintext";
  const id = "id_" + Math.random().toString(36).slice(2);
  (window as any)[id] = () => {
    writeToClipboard(code);
    message.success(t("common.copy.success"));
  }
  return `
    <div class="code-wrapper relative">
      <pre style="padding: 1.5rem 1rem !important;"><code lang="${language}" class="text-[var(--code-block-color)]">${
    hljs.highlight(code, { language }).value
  }</code></pre>
      <div class="absolute top-0 left-0 rounded py-1 px-[0.5rem] cursor-pointer text-xs text-gray-500 text-[var(--code-block-lang-color)]"
      onclick="${id}()"
      >
        ${t("common.copy")}
      </div>
      <div class="absolute top-0 right-0 py-1 px-[0.5rem] cursor-pointer text-xs text-gray-500 text-[var(--code-block-lang-color)] bg-[var(--code-block-lang-bg-color)] rounded-tr-md rounded-bl-md">
        ${language}
      </div>
    </div>
    `;
};
renderer.html = (html) => {
  return `<p>${escapeHtml(html)}</p>`;
};

export function renderMarkdown(md: string) {
  return marked(md, { renderer });
}

function escapeHtml(str: string) {
  let div = document.createElement("div");
  div.innerText = str;
  return div.innerHTML;
}

export default marked.parse;
