// @ts-ignore
import "./style.css";
import Alpine from "alpinejs";
import HTMX from "htmx.org";
import highlight from "highlight.js";

declare global {
  interface Window {
    Alpine: typeof Alpine;
    htmx: typeof HTMX;
    highlight: typeof highlight.highlight;
  }
}

window.Alpine = Alpine;
window.htmx = HTMX;
window.highlight = (content: string) =>
  highlight.highlight(content, { language: "markdown" });

import markdown from "highlight.js/lib/languages/markdown";

highlight.registerLanguage("markdown", markdown);

Alpine.start();
