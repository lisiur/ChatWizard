import { invoke } from "./api";

export function interceptLink(dom: HTMLElement) {
  dom.querySelectorAll("a").forEach(function (a) {
    if (a.href.startsWith("http")) {
      a.addEventListener("click", function (e) {
        e.preventDefault();
        invoke("open", { url: a.href });
      });
    }
  });
}
