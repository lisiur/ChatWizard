import { ref } from "vue";
import { hideWindow, currentWindow } from "../utils/api";
import { isTauri } from "../utils/env";

export function useHideWindowWhenBlur(params?: {
  onBlur?: () => void;
  onFocus?: () => void;
}) {
  let enabled = ref(true);

  if (isTauri) {
    currentWindow().listen("tauri://blur", () => {
      params?.onBlur?.();
      if (enabled.value) {
        hideWindow();
      }
    });
    currentWindow().listen("tauri://focus", () => {
      params?.onFocus?.();
    });
  }

  function enable() {
    enabled.value = true;
  }

  function disable() {
    enabled.value = false;
  }

  function toggleEnable() {
    enabled.value = !enabled.value;
  }

  return {
    enable,
    disable,
    enabled,
    toggleEnable,
  };
}
