import { window } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { Theme } from "../api";
import { setTheme as _setTheme } from "../themes";

export async function setTheme(theme: Theme) {
  let unListen: UnlistenFn | undefined;
  if (theme === "system") {
    window
      .getCurrent()
      .theme()
      .then((theme) => {
        _setTheme(theme ?? "light");
      });
    unListen = await listen("tauri://theme-changed", (payload) => {
      const theme = payload.payload as "light" | "dark";
      _setTheme(theme);
    });
  } else {
    if (unListen) {
      unListen();
    }
    _setTheme(theme);
  }
}
