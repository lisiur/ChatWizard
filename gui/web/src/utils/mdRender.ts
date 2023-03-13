import { marked } from "marked";
import hljs from 'highlight.js'
import "highlight.js/styles/monokai.css"

marked.setOptions({
    highlight(code, lang) {
        const language = hljs.getLanguage(lang) ? lang : "plaintext";
        return hljs.highlight(code, { language }).value;
    },
})

export default marked.parse