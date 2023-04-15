import enUS, { Messages } from "./enUS";
import zhCN from "./zhCN";
import ruRU from "./ruRU";

// 1. import new language above

export type { Messages };

export const languages = [
  {
    lang: "enUS",
    messages: enUS,
  },
  {
    lang: "zhCN",
    messages: zhCN,
  },
  {
    lang: "ruRU",
    messages: ruRU,
  },

  // 2. Add more languages above
];

export default Object.fromEntries(
  languages.map((item) => [item.lang, item.messages])
);
