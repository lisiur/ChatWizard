import { computed, reactive, ref, Ref, toRef, toRefs } from "vue";
import { vars as lightVars, override as overrideLightVars } from "./light/vars";
import darkVars, { override as overrideDarkVars } from "./dark/vars";
import type { ThemeVars } from "./types";

// ---------- utils
function camelToKebabCase(str: string) {
  return str.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
}

// cache vars
const vars = reactive<{ [k: string]: string }>({});

// update vars if documentElement changed
const observer = new MutationObserver(() => {
  const styles = getComputedStyle(document.documentElement);
  for (const v of Object.keys(vars)) {
    vars[v] = styles.getPropertyValue(v);
  }
});

// watch changes
observer.observe(document.documentElement, {
  attributeFilter: ["style"],
  attributes: true,
});

export function useThemeVar(name: string): Ref<string> {
  if (!vars[name]) {
    vars[name] = getComputedStyle(document.documentElement).getPropertyValue(
      name
    );
  }
  return toRef(vars, name);
}

export const themeVars = reactive({
  light: lightVars,
  dark: darkVars,
});

const currentTheme = ref<keyof typeof themeVars>("light");

const currentThemeVars = computed(() => {
  return themeVars[currentTheme.value];
});

export function getTheme() {
  return currentTheme.value;
}

function setThemeVar(key: string, val: string) {
  key = camelToKebabCase(key);
  if (!key.startsWith("--")) {
    key = "--" + key;
  }
  document.documentElement.style.setProperty(key, val);
}

function setThemeVars(vars: Record<string, string | undefined>) {
  Object.entries(vars).forEach(([key, val]) => {
    if (val) {
      setThemeVar(key, val);
    }
  });
}

export type ThemeLiteral = keyof typeof themeVars;
export function setTheme(theme: ThemeLiteral) {
  currentTheme.value = theme;
  setThemeVars(themeVars[theme].common!);
  setThemeVars(themeVars[theme].custom!);
}

export function useThemeVars() {
  return toRefs(currentThemeVars);
}

const naiveUiOverrides = reactive({
  light: lightVars,
  dark: darkVars,
});
export function getNaiveUiThemeOverrides() {
  return computed(() => {
    return naiveUiOverrides[currentTheme.value];
  });
}

export function overrideTheme(theme: { light?: ThemeVars; dark?: ThemeVars }) {
  if (theme.light) {
    overrideLightVars(theme.light);
  }
  if (theme.dark) {
    overrideDarkVars(theme.dark);
  }
}

export const theme = currentTheme;

export function useTheme() {
  return {
    theme,
    vars: useThemeVars(),
  };
}

export { ThemeVars };
