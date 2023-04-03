import { message } from "../utils/prompt";
import { invoke as _invoke } from "@tauri-apps/api";
import { i18n } from "../hooks/i18n";

const { t } = i18n.global;

const invoke = async <T>(...args: Parameters<typeof _invoke>) => {
  return _invoke<T>(...args).catch((err) => {
    console.log(err);
    const msg: string = err.toString();
    let errMsg = (() => {
      if (msg.startsWith("timeout")) {
        return t("common.network.error.timeout");
      } else if (msg.startsWith("connect")) {
        return t("common.network.error.connect");
      } else {
        return msg;
      }
    })();
    message.error(errMsg);
    debugLog(err.toString());
    return Promise.reject(errMsg);
  });
};

export interface ChatIndex {
  id: string;
  title: string;
  promptId?: string;
  config: ChatConfig;
  cost: number;
  vendor: string;
  sort: number;
  stick: boolean;
  archive: boolean;
  createdAt: string;
  updatedAt: string;
  archivedAt?: string;
}

export interface ChatLog {
  id: string;
  chatId: string;
  role: "system" | "user" | "assistant";
  message: string;
  model: string;
  tokens: number;
  cost: number;
  createdAt: string;
  updatedAt: string;
}

export interface ChatUpdatePayload {
  id: string;
  title?: string;
  promptId?: string;
  config?: ChatConfig;
}

export interface ChatConfig {
  backtrack: number;
  params: {
    model?: string;
    temperature?: number;
    stop?: Array<string>;
    presencePenalty?: number;
    frequencyPenalty?: number;
  };
}

export interface ChatModel {
  id: string;
  name: string;
  description: string;
  price: number;
  unit: string;
  vendor: string;
}

export interface PromptIndex {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

export interface PromptData {
  id: string;
  name: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

export interface PromptMarketSource {
  id: string;
  name: string;
  description: string;
  url: string;
  type: string;
}

export interface MarketPrompt {
  name: string;
  content: string;
}

export type PromptUpdatePayload = {
  id: string;
  name?: string;
  content?: string;
};

export interface Settings {
  apiKey?: string;
  proxy?: string;
  theme?: Theme;
  language?: string;
  forwardUrl?: string;
  forwardApiKey?: boolean;
}

export enum Theme {
  Light = "light",
  Dark = "dark",
  System = "system",
}

export interface WindowOptions {
  title: string;
  url: string;
  width: number;
  height: number;
  resizable: boolean;
  alwaysOnTop: boolean;
  visible: boolean;
  minSize?: [number, number];
  maxSize?: [number, number];
}

export async function getChat(id: string) {
  return invoke<ChatIndex>("get_chat", { id });
}

export async function allChats() {
  return invoke<Array<ChatIndex>>("all_chats");
}

export async function loadChat(chatId: string) {
  return invoke<Array<ChatLog>>("load_chat", { chatId });
}

export async function loadChatLogByCursor(params: {
  chatId: string;
  cursor?: string;
  size: number;
}) {
  debugLog("load_chat_log_by_cursor: " + params.cursor?.slice(-2));
  const res = await invoke<{
    records: Array<ChatLog>;
    nextCursor: string | null;
  }>("load_chat_log_by_cursor", { ...params });
  debugLog(
    "load_chat_log_by_cursor_result: \n -> " +
      res.nextCursor?.slice(-2) +
      "\n" +
      res.records.map((it) => it.id.slice(-2) + " " + it.message).join("\n")
  );

  return res;
}

export async function updateChat(payload: ChatUpdatePayload) {
  return invoke<void>("update_chat", { payload });
}

export async function newChat(params?: { promptId?: string; title?: string }) {
  return invoke<string>("new_chat", params);
}

export async function deleteChat(chatId: string) {
  return invoke<void>("delete_chat", { chatId });
}

export async function allNonStickChats() {
  return invoke<Array<ChatIndex>>("all_non_stick_chats");
}

export async function allStickChats() {
  return invoke<Array<ChatIndex>>("all_stick_chats");
}

export async function allArchiveChats() {
  return invoke<Array<ChatIndex>>("all_archive_chats");
}

export async function setChatArchive(chatId: string) {
  return invoke<void>("set_chat_archive", { chatId });
}

export async function setChatStick(chatId: string, stick: boolean) {
  return invoke<void>("set_chat_stick", { chatId, stick });
}

export async function moveStickChat(from: string, to: string) {
  return invoke<void>("move_stick_chat", { from, to });
}

export async function moveNonStickChat(from: string, to: string) {
  return invoke<void>("move_non_stick_chat", { from, to });
}

export async function deleteChatLog(logId: string) {
  return invoke<void>("delete_chat_log", { logId });
}

export function sendMessage(chatId: string, message: string) {
  return invoke<[string, string]>("send_message", { chatId, message });
}

export function resendMessage(messageId: string) {
  return invoke<[string, string]>("resend_message", { messageId });
}

export function getChatModels() {
  return invoke<Array<ChatModel>>("get_chat_models");
}

export function allPrompts() {
  return invoke<Array<PromptIndex>>("all_prompts");
}

export function createPrompt(prompt: { name: string; content: string }) {
  return invoke<string>("create_prompt", prompt);
}

export function updatePrompt(payload: PromptUpdatePayload) {
  return invoke("update_prompt", { payload });
}

export function deletePrompt(id: string) {
  return invoke("delete_prompt", { id });
}

export function loadPrompt(id: string) {
  return invoke<PromptData>("load_prompt", { id });
}

export function getPromptSources() {
  return invoke<Array<PromptMarketSource>>("get_prompt_sources");
}

export function getPromptSourcePrompts(sourceId: string) {
  return invoke<Array<MarketPrompt>>("get_prompt_source_prompts", {
    sourceId,
  });
}

export function installMarketPrompt(prompt: MarketPrompt) {
  return invoke<string>("install_market_prompt", { ...prompt });
}

export function installMarketPromptAndCreateChat(prompt: MarketPrompt) {
  return invoke<string>("install_market_prompt_and_create_chat", { ...prompt });
}

export function getSettings() {
  return invoke<Settings>("get_settings");
}

export function updateSettings(payload: Settings) {
  return invoke<void>("update_settings", { payload });
}

export function getTheme() {
  return invoke<Theme>("get_theme");
}

export function checkApiKey() {
  return invoke<void>("has_api_key");
}

export function getProxy() {
  return invoke<string>("get_proxy");
}

export function hasApiKey() {
  return invoke<boolean>("has_api_key");
}

export function getLocale() {
  return invoke<string>("get_locale");
}

export function exportMarkdown(chatId: string, path: string) {
  return invoke<void>("export_markdown", {
    chatId,
    path,
  });
}

export function showWindow(label: string) {
  return invoke<void>("show_window", { label });
}

export function createWindow(label: string, options: WindowOptions) {
  return invoke<void>("create_window", { label, options });
}

export function showOrCreateWindow(label: string, options: WindowOptions) {
  return invoke<void>("show_or_create_window", { label, options });
}

export function debugLog(log: string) {
  return invoke<void>("debug_log", { log });
}
