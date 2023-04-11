import { zhCN, dateZhCN, enUS, dateEnUS } from "naive-ui";
import { ref, computed } from "vue";
import { createI18n, useI18n as _useI18n } from "vue-i18n";
import messages, { Messages } from "../i18n";

export const Lang: Record<string, any> = {
  zhCN,
  enUS,
};

const DateLang: Record<string, any> = {
  zhCN: dateZhCN,
  enUS: dateEnUS,
};

const currentLang = ref<string>("enUS");
const currentNaiveUiLang = computed(() => {
  return Lang[currentLang.value];
});
const currentNaiveUiDateLang = computed(() => {
  return DateLang[currentLang.value];
});

const i18n = createI18n({
  locale: "enUS",
  fallbackLocale: "enUS",
  globalInjection: true,
  allowComposition: true,
  messages,
});

function setLocale(lang: string) {
  currentLang.value = lang;
  i18n.global.locale = lang as any;
}

function useI18n() {
  const { t: _t } = _useI18n();
  const t = (key: keyof Messages, options?: any) => {
    return _t(key, options);
  };

  return {
    t,
  };
}

export {
  i18n,
  useI18n,
  setLocale,
  currentLang,
  currentNaiveUiLang,
  currentNaiveUiDateLang,
};
