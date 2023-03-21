import { invoke } from "@tauri-apps/api";

export interface ChatMetadata {
  id: string;
  title: string;
  prompt_id?: string;
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
  config: ChatConfig;
}

export interface ChatUpdatePayload {
  id: string;
  title?: string;
  promptId?: string;
  config?: ChatConfig;
}

export interface ChatConfig {
  model?: string;
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

export interface PromptMetadata {
  id: string;
  act: string;
}

export interface Prompt {
  id: string;
  act: string;
  prompt: string;
}

export type PromptUpdatePayload = Partial<Prompt> & { id: string };

export interface Settings {
  apiKey?: string;
  orgId?: string;
  proxy?: string;
  theme?: Theme;
  locale?: string;
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
  return invoke<Array<ChatMetadata>>("all_chats");
}

export async function readChat(chatId: string) {
  return invoke<ChatData>("load_chat", { chatId });
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
  return invoke<void>("resend_message", { chatId, messageId });
}

export function allPrompts() {
  return invoke<Array<PromptMetadata>>("all_prompts");
}

export function createPrompt(prompt: Omit<Prompt, "id">) {
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
  return invoke<Prompt>("load_prompt", { id });
}

export function getSettings() {
  return invoke<Settings>("get_settings");
}

export function setTheme(theme: Theme) {
  return invoke<void>("set_theme", { theme });
}

export function getTheme() {
  return invoke<Theme>("get_theme");
}

export function setApiKey(apiKey: string) {
  return invoke<void>("set_api_key", { apiKey });
}

export function checkApiKey() {
  return invoke<void>("has_api_key");
}

export function setProxy(proxy: string) {
  return invoke<void>("set_proxy", { proxy });
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

export function setLocale(locale: string) {
  return invoke<void>("set_locale", { locale });
}

export function saveAsMarkdown(chatId: string, path: string) {
  return invoke<void>("save_as_markdown", {
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
