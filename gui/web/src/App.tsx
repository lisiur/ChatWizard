import { defineComponent, onBeforeUnmount, onMounted, ref } from "vue";
import { RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui";
import { configProviderProps } from "./config";
import { getTheme, Theme, showWindow, getLocale, debugLog } from "./api";
import { setTheme } from "./utils/theme";
import { useRoute } from "vue-router";
import { window } from "@tauri-apps/api";
import { setupLifeCycle } from "./utils/setupLifeCycle";
import { setLocale } from "./hooks/i18n";

export default defineComponent({
  setup() {
    const route = useRoute();
    const windowLabel = route.path.split("/")[1];
    debugLog(`window label: ${windowLabel}`);

    setupLifeCycle()
      .onMounted((ctx) => {
        getTheme().then(async (theme) => {
          debugLog(`theme: ${theme}`);
          setTheme(theme ?? Theme.System);

          // show window after theme is set
          // to avoid flash of unstyled content
          if (windowLabel) {
            debugLog(`show window: ${windowLabel}`);
            showWindow(windowLabel);
          }

          const unListen = await window
            .getCurrent()
            .listen("theme-changed", (e) => {
              setTheme(e.payload as Theme);
            });
          ctx.onBeforeUnmount(unListen);
        });
      })
      .onMounted((ctx) => {
        getLocale().then(async (_locale) => {
          const locale = _locale || "enUS";
          setLocale(locale);
          const unListen = await window
            .getCurrent()
            .listen("locale-changed", (e) => {
              setLocale(e.payload as string);
            });
          ctx.onBeforeUnmount(unListen);
        });
      })
      .setup();

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
