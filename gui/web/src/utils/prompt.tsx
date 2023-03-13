import { configProviderProps } from "../config";
import { createDiscreteApi, NInput, NButton, NSpace } from "naive-ui";
import { ref } from "vue";

const { message, notification, dialog, loadingBar } = createDiscreteApi(
  ["message", "dialog", "notification", "loadingBar"],
  {
    configProviderProps,
    loadingBarProviderProps: {
      loadingBarStyle: {
        loading: "background-color: var(--loading-bar-bg-color)",
      },
    },
  }
);

async function prompt(
  title: string,
  config?: {
    defaultValue?: string;
    showCancel?: boolean;
  }
): Promise<string> {
  const showCancel = config?.showCancel ?? true;
  return new Promise((resolve, reject) => {
    const value = ref(config?.defaultValue || "");
    const { destroy } = dialog.create({
      type: "default",
      closable: false,
      autoFocus: true,
      showIcon: false,
      maskClosable: false,
      title,
      content: () => <NInput v-model:value={value.value}></NInput>,
      action: () => (
        <NSpace>
          {showCancel ? (
            <NButton onClick={cancelHandler}>Cancel</NButton>
          ) : null}
          <NButton type="primary" onClick={okHandler}>
            Ok
          </NButton>
        </NSpace>
      ),
    });
    function cancelHandler() {
      destroy();
      reject();
    }
    function okHandler() {
      destroy();
      resolve(value.value);
    }
  });
}

export { message, notification, dialog, loadingBar, prompt };
