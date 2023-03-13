import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { prompt } from "../utils/prompt";

async function has_api_key(): Promise<boolean> {
  return invoke("has_api_key");
}

async function set_api_key(apiKey: string): Promise<void> {
  return invoke("set_api_key", { apiKey });
}

export function useConfig() {
  const _has_api_key = ref(false);

  async function checkApiKey() {
    if (_has_api_key.value) {
      return;
    }
    _has_api_key.value = await has_api_key();

    if (!_has_api_key.value) {
      let api_key = null as string | null;
      while (!api_key) {
        api_key = await prompt("Please input api key:", {
          showCancel: false,
        });
      }
      await set_api_key(api_key);
    }
  }

  async function setApiKey() {
    let api_key = null as string | null;
    while (!api_key) {
      api_key = await prompt("Please input api key:");
    }
    await set_api_key(api_key);
  }

  async function setProxy() {
    let currentProxy = ((await invoke("get_proxy")) as string) ?? "";
    const proxy = await prompt("Please input proxy:", {
      defaultValue: currentProxy,
    });
    await invoke("set_proxy", { proxy });
  }

  return {
    checkApiKey,
    setProxy,
    setApiKey,
  };
}
