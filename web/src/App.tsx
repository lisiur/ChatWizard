import { defineComponent } from "vue";
import { RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui";
import { configProviderProps } from "./config";
import { getTheme, Theme, getLocale, showWindow } from "./api";
import { setTheme } from "./utils/theme";
import { useRoute } from "vue-router";
import { setupLifeCycle } from "./utils/setupLifeCycle";
import { setLocale } from "./hooks/i18n";
import { listen } from "./utils/api";
import { isTauri } from "./utils/env";

export default defineComponent({
  setup() {
    const route = useRoute();
    const windowLabel = route.path.split("/")[1];

    setupLifeCycle()
      .onMounted((ctx) => {
        getTheme().then(async (theme) => {
          setTheme(theme ?? Theme.System);

          if (isTauri) {
            // show window after theme is set
            // to avoid flash of unstyled content
            if (windowLabel) {
              showWindow(windowLabel);
            }

            const unListen = await listen("theme-changed", (e) => {
              console.log(e);
              setTheme(e.payload as Theme);
            });
            ctx.onBeforeUnmount(unListen);
          }
        });
      })
      .onMounted((ctx) => {
        getLocale().then(async (_locale) => {
          const locale = _locale || "enUS";
          setLocale(locale);

          if (isTauri) {
            const unListen = await listen("locale-changed", (e) => {
              setLocale(e.payload as string);
            });
            ctx.onBeforeUnmount(unListen);
          }
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
        {...configProviderProps}
      >
        <RouterView />
      </NConfigProvider>
    );
  },
});
