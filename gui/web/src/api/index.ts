import { message } from "../utils/prompt";
import { invoke as _invoke } from "@tauri-apps/api";
import { i18n } from "../hooks/i18n";

const { t } = i18n.global;

const invoke = async <T>(...args: Parameters<typeof _invoke>) => {
  return _invoke<T>(...args).catch((err) => {
    const msg: string = err.toString();
    if (msg.includes("timed out")) {
      message.error(t("common.network.timeout"));
    }
    return Promise.reject(err);
  });
};

export interface ChatIndex {
  id: string;
  title: string;
}

export interface ChatMetadata {
  config: ChatConfig;
}

export interface ChatData {
  logs: Array<{
    id: string;
    message: {
      role: "system" | "user" | "assistant";
      content: string;
    };
  }>;
  cost: number;
}

export interface ChatUpdatePayload {
  id: string;
  title?: string;
  config?: ChatConfig;
}

export interface ChatConfig {
  model?: string;
  promptId?: string;
  maxBacktrack?: number;
  temperature?: number;
  topP?: number;
  n?: number;
  stop?: Array<string>;
  maxTokens?: number;
  presencePenalty?: number;
  frequencyPenalty?: number;
  user?: string;
}

export interface PromptIndex {
  id: string;
  act: string;
}

export interface PromptMetadata {
  act: string;
}

export interface PromptData {
  prompt: string;
}

export interface PromptMarketRepo {
  name: string;
  url: string;
}

export interface MarketPromptIndex {
  id: string;
  act: string;
}

export interface MarketPrompt {
  act: string;
  prompt: string;
  author?: string;
}

export type PromptUpdatePayload = { id: string; act?: string; prompt?: string };

export interface Settings {
  apiKey?: string;
  orgId?: string;
  proxy?: string;
  theme?: Theme;
  locale?: string;
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
  url?: string;
  width: number;
  height: number;
}

export async function allChats() {
  return invoke<Array<ChatIndex>>("all_chats");
}

export async function readChat(chatId: string) {
  return invoke<[ChatMetadata, ChatData]>("load_chat", { chatId });
}

export async function updateChat(payload: ChatUpdatePayload) {
  return invoke<void>("update_chat", { payload });
}

export async function createChat(params?: {
  promptId?: string;
  title?: string;
}) {
  return invoke<string>("new_chat", params);
}

export async function deleteChat(chatId: string) {
  return invoke<void>("delete_chat", { chatId });
}

export function sendMessage(chatId: string, message: string) {
  return invoke<string>("send_message", { chatId, message });
}

export function resendMessage(chatId: string, messageId: string) {
  return invoke<string>("resend_message", { chatId, messageId });
}

export function allPrompts() {
  return invoke<Array<PromptIndex>>("all_prompts");
}

export function createPrompt(prompt: { act: string; prompt: string }) {
  return invoke<string>("create_prompt", {
    act: prompt.act,
    prompt: prompt.prompt,
  });
}

export function updatePrompt(payload: PromptUpdatePayload) {
  return invoke("update_prompt", { payload });
}

export function deletePrompt(id: string) {
  return invoke("delete_prompt", { id });
}

export function loadPrompt(id: string) {
  return invoke<[PromptMetadata, PromptData]>("load_prompt", { id });
}

export function allRepos() {
  return invoke<Array<PromptMarketRepo>>("all_repos");
}

export function repoIndexList(name: string) {
  return invoke<Array<MarketPromptIndex>>("repo_index_list", { name });
}

export function loadMarketPrompt(id: string, name: string) {
  return invoke<MarketPrompt>("load_market_prompt", { id, name });
}

export function installPrompt(prompt: MarketPrompt) {
  return invoke("install_prompt", { prompt });
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

export function showWindow(label: string, options?: WindowOptions) {
  return invoke<void>("show_window", { label, options });
}

export function debugLog(log: string) {
  return invoke<void>("debug_log", { log });
}
