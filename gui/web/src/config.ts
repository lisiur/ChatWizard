import { ConfigProviderProps, darkTheme, lightTheme } from "naive-ui";
import { computed } from "vue";
import { getTheme, getNaiveUiThemeOverrides } from "./themes";

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
    } as ConfigProviderProps)
);
