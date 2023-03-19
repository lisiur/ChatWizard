import { ConfigProviderProps, darkTheme, lightTheme } from "naive-ui";
import { computed } from "vue";
import { getTheme, getNaiveUiThemeOverrides } from "./themes";
import { currentNaiveUiDateLang, currentNaiveUiLang } from "./hooks/i18n";

const naiveTheme = computed(() => {
  if (getTheme() === "dark") {
    return darkTheme;
  } else {
    return lightTheme;
  }
});
const naiveUiOverrides = getNaiveUiThemeOverrides();
export const configProviderProps = computed(
  () =>
    ({
      theme: naiveTheme.value,
      themeOverrides: naiveUiOverrides,
      locale: currentNaiveUiLang.value,
      dateLocale: currentNaiveUiDateLang.value,
    } as ConfigProviderProps)
);
