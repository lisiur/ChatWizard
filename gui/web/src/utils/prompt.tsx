import { configProviderProps } from "../config";
import { createDiscreteApi, NInput, NButton, NSpace } from "naive-ui";
import { ref, watch } from "vue";
import { MessageApiInjection } from "naive-ui/es/message/src/MessageProvider";
import { NotificationApiInjection } from "naive-ui/es/notification/src/NotificationProvider";
import { DialogApiInjection } from "naive-ui/es/dialog/src/DialogProvider";
import { LoadingBarApiInjection } from "naive-ui/es/loading-bar/src/LoadingBarProvider";

let message!: MessageApiInjection;
let notification!: NotificationApiInjection;
let dialog!: DialogApiInjection;
let loadingBar!: LoadingBarApiInjection;

watch(
  configProviderProps,
  () => {
    let api = createDiscreteApi(
      ["message", "dialog", "notification", "loadingBar"],
      {
        configProviderProps: configProviderProps.value,
        loadingBarProviderProps: {
          loadingBarStyle: {
            loading: "background-color: var(--loading-bar-bg-color)",
          },
        },
      }
    );

    message = api.message;
    notification = api.notification;
    dialog = api.dialog;
    loadingBar = api.loadingBar;
  },
  {
    immediate: true,
  }
);

async function prompt(
  title: string,
  config?: {
    defaultValue?: string;
    showCancel?: boolean;
    okHandler?: (
      value: string
    ) => void | undefined | boolean | Promise<void | undefined | boolean>;
  }
): Promise<string> {
  const loading = ref(false);
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
          <NButton type="primary" onClick={okHandler} loading={loading.value}>
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
      const result = config?.okHandler?.(value.value);
      if (result instanceof Promise) {
        loading.value = true;
        result
          .then((close) => {
            if (close !== false) {
              destroy();
              resolve(value.value);
            }
          })
          .finally(() => {
            loading.value = false;
          });
      } else {
        if (result !== false) {
          destroy();
          resolve(value.value);
        }
      }
    }
  });
}

export { message, notification, dialog, loadingBar, prompt };
