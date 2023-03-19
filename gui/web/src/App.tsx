import { defineComponent, onBeforeUnmount, onMounted } from "vue";
import { RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui";
import { configProviderProps } from "./config";
import { getTheme, Theme, showWindow, debugLog } from "./api";
import { setTheme } from "./utils/theme";
import { useRoute } from "vue-router";
import { window } from "@tauri-apps/api";
import { UnlistenFn } from "@tauri-apps/api/event";

export default defineComponent({
  setup() {
    const route = useRoute();
    const windowLabel = route.path.split("/")[1];

    let unListen: UnlistenFn;

    onMounted(() => {
      getTheme().then(async (theme) => {
        setTheme(theme ?? Theme.System);
        if (windowLabel) {
          unListen = await window.getCurrent().listen("theme-changed", (e) => {
            debugLog(`window: ${windowLabel} change theme to ${e.payload}`);
            setTheme(e.payload as Theme);
          });
          debugLog(`current window: ${windowLabel}`);
          showWindow(windowLabel);
        }
      });
    });

    onBeforeUnmount(() => {
      if (unListen) {
        unListen();
      }
    });

    return () => (
      <NConfigProvider
        class="h-full"
        style={{
          backgroundColor: "var(--body-color)",
          color: "var(--text-color-base)",
        }}
        {...configProviderProps.value}
      >
        <RouterView />
      </NConfigProvider>
    );
  },
});
