import { invoke } from "./api";
import { isTauri } from "./env";

export function interceptLink(dom: HTMLElement) {
  dom.querySelectorAll("a").forEach(function (a) {
    if (a.href.startsWith("http")) {
      a.addEventListener("click", function (e) {
        e.preventDefault();
        if (isTauri) {
          invoke("open", { url: a.href });
        } else {
          window.open(a.href, "_blank");
        }
      });
    }
  });
}
