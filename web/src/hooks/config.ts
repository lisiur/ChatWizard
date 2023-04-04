import { ref } from "vue";
import { prompt, message } from "../utils/prompt";
import { invoke } from "../utils/api";

async function _hasApiKey(): Promise<boolean> {
  return invoke("has_api_key");
}

async function _setApiKey(apiKey: string): Promise<string> {
  return invoke<any>("set_api_key", { apiKey })
    .then(() => "")
    .catch((err) => {
      return err;
    });
}

export function useConfig() {
  const _has_api_key = ref(false);

  async function checkApiKey() {
    if (_has_api_key.value) {
      return;
    }
    _has_api_key.value = await _hasApiKey();

    if (!_has_api_key.value) {
      await setApiKey();
    }
  }

  async function setApiKey() {
    await prompt("Please input api key:", {
      async okHandler(apiKey) {
        if (apiKey === "") {
          message.error("api key can not be empty");
          return false;
        }
        const errMsg = await _setApiKey(apiKey);
        if (errMsg) {
          message.error(errMsg);
          return false;
        }
      },
    });
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
