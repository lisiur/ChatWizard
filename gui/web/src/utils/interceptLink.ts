import { invoke } from "@tauri-apps/api";
import { Ref, watch } from "vue";

export function interceptLink(
  dom: Ref<HTMLElement | undefined>,
  done: Ref<boolean>
) {
  watch(
    () => [dom.value, done.value],
    () => {
      if (dom.value && done.value) {
        if (dom.value.dataset.intercepted) return;
        injectListener(dom.value);
        dom.value.dataset.intercepted = "true";
      }
    },
    {
      immediate: true,
    }
  );
}

function injectListener(dom: HTMLElement) {
  dom.querySelectorAll("a").forEach(function (a) {
    a.addEventListener("click", function (e) {
      e.preventDefault(); // 阻止默认行为
      invoke("open", { url: a.href });
    });
  });
}
