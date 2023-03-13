import { defineComponent, onMounted } from "vue";
import { RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui";
import { configProviderProps } from "./config";
import { invoke } from "@tauri-apps/api";

export default defineComponent({
  setup() {
    onMounted(() => {
      invoke("show_main_window")
    })

    return () => (
      <NConfigProvider class="h-full" {...configProviderProps}>
        <RouterView />
      </NConfigProvider>
    );
  },
});
