import "./style.css";
import Alpine from "alpinejs";
import HTMX from "htmx.org";

declare global {
  interface Window {
    Alpine: typeof Alpine;
    htmx: typeof HTMX;
  }
}

window.Alpine = Alpine;
window.htmx = HTMX;

Alpine.start();
