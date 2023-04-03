import { darkTheme, lightTheme } from "naive-ui";
import { computed, reactive } from "vue";
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
export const configProviderProps = reactive({
  theme: naiveTheme,
  themeOverrides: naiveUiOverrides,
  locale: currentNaiveUiLang,
  dateLocale: currentNaiveUiDateLang,
});
