import enUS, { Messages } from "./enUS";
import zhCN from "./zhCN";
import ruRU from "./ruRU";

export type { Messages };

export const languages = [
  {
    name: "enUS",
    messages: enUS,
  },
  {
    name: "zhCN",
    messages: zhCN,
  },
  {
    name: "ruRU",
    messages: ruRU,
  },
];

export default Object.fromEntries(
  languages.map((item) => [item.name, item.messages])
);
