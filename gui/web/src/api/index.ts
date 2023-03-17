import { invoke } from "@tauri-apps/api";

export interface ChatMetadata {
  id: string;
  title: string;
  act?: string;
}

export interface ChatData {
  id: string;
  title: string;
  prompt?: string;
  logs: Array<{
    id: string;
    message: {
      role: "system" | "user" | "assistant";
      content: string;
    };
  }>;
}

export interface Prompt {
  act: string;
  prompt: string;
}

export async function allChats() {
  return invoke<Array<ChatMetadata>>("all_chats");
}

export async function readChat(chatId: string) {
  return invoke<ChatData>("load_chat", { chatId });
}

export async function createChat(params?: { act?: string; title?: string }) {
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
  return invoke<
    Array<{
      act: string;
    }>
  >("all_prompts");
}

export function createPrompt(prompt: Prompt) {
  return invoke("create_prompt", { prompt });
}

export function updatePrompt(prompt: Prompt) {
  return invoke("update_prompt", { prompt });
}

export function deletePrompt(act: string) {
  return invoke("delete_prompt", { act });
}

export function loadPrompt(act: string) {
  return invoke<Prompt>("load_prompt", { act });
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

export function showMainWindow() {
  return invoke<void>("show_main_window");
}

export function saveAsMarkdown(chatId: string, path: string) {
  return invoke<void>("save_as_markdown", {
    chatId,
    path,
  });
}
