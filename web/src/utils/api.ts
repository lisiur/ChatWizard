import {
  os,
  dialog,
  invoke as _invoke,
  event,
  clipboard,
  window as tauriWindow,
  app,
  updater,
} from "@tauri-apps/api";
import { i18n } from "../hooks/i18n";
import { message } from "./prompt";
import { EventCallback, EventName } from "@tauri-apps/api/event";
import { clientId, listen as wsListen } from "./websocket";
import { isWeb } from "./env";
const { t } = i18n.global;

export async function getPlatform() {
  if (isWeb) {
    return "web";
  }
  return await os.platform();
}

export async function save(...params: Parameters<typeof dialog.save>) {
  if (isWeb) {
    throw new Error("save not supported in web");
  }
  return dialog.save(...params);
}

export async function execCommand<T>(command: string, payload = null as any) {
  if (isWeb) {
    return fetch("/api/command", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-client-id": clientId,
      },
      body: JSON.stringify({
        command,
        payload,
      }),
    })
      .then((res) => {
        if (res.status >= 300) {
          return res.json().then((error) => {
            return Promise.reject(error);
          });
        }
        return res.json() as T;
      })
      .catch((err) => {
        if (err instanceof Response) {
          return err.text().then((text) => {
            return Promise.reject(text);
          });
        } else {
          throw err;
        }
      });
  } else {
    return invoke<T>("exec_command", {
      command,
      payload,
    });
  }
}

export async function invoke<T>(
  command: string,
  payload?: Record<string, any>
) {
  return _invoke<T>(command, payload).catch((err) => {
    const msg: string = err.toString();
    let errMsg = (() => {
      if (msg.startsWith("timeout")) {
        return t("common.network.error.timeout");
      } else if (msg.startsWith("connect")) {
        return t("common.network.error.connect");
      } else {
        return msg;
      }
    })();
    message.error(errMsg);
    return Promise.reject(errMsg);
  });
}

export async function listen<T>(
  eventName: EventName,
  eventHandler: EventCallback<T>
) {
  if (isWeb) {
    return wsListen(eventName, eventHandler);
  }
  return event.listen(eventName, eventHandler);
}

export async function once<T>(
  eventName: EventName,
  eventHandler: EventCallback<T>
) {
  if (isWeb) {
    throw new Error("once not supported in web");
  }
  return event.once(eventName, eventHandler);
}

export async function hideWindow() {
  if (isWeb) {
    throw new Error("hideWindow not supported in web");
  }
  return currentWindow().hide();
}

export async function emit(eventName: string, payload?: unknown) {
  if (isWeb) {
    throw new Error("emit not supported in web");
  }

  return event.emit(eventName);
}

export async function writeToClipboard(content: string) {
  if (isWeb) {
    throw new Error("writeToClipboard not supported in web");
  }

  await clipboard.writeText(content);
}

export async function getSystemTheme() {
  if (isWeb) {
    const darkThemeQuery = window.matchMedia("(prefers-color-scheme: dark)");
    if (darkThemeQuery.matches) {
      return "dark";
    } else {
      return "light";
    }
  }

  return tauriWindow.getCurrent().theme();
}

export async function getVersion() {
  if (isWeb) {
    throw new Error("getVersion not supported in web");
  }
  return app.getVersion();
}

export async function checkUpdate() {
  if (isWeb) {
    throw new Error("checkUpdate not supported in web");
  }
  return updater.checkUpdate();
}

export function currentWindow() {
  if (isWeb) {
    throw new Error("currentWindow not supported in web");
  }
  return tauriWindow.getCurrent();
}

export function openUrl(url: string) {
  if (isWeb) {
    window.open(url, "_blank");
  } else {
    invoke("open", { url });
  }
}
