import { defineComponent, onMounted } from "vue";
import { RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui";
import { configProviderProps } from "./config";
import { invoke } from "@tauri-apps/api";
import { setTheme } from "./themes";
import { listen } from "@tauri-apps/api/event";
import { window } from "@tauri-apps/api";

export default defineComponent({
  setup() {
    window
      .getCurrent()
      .theme()
      .then((theme) => {
        setTheme(theme ?? "light");
      });
    listen("tauri://theme-changed", (payload) => {
      const theme = payload.payload as "light" | "dark";
      setTheme(theme);
    });

    onMounted(() => {
      invoke("show_main_window");
    });

    return () => (
      <NConfigProvider class="h-full" {...configProviderProps.value}>
        <RouterView />
      </NConfigProvider>
    );
  },
});
