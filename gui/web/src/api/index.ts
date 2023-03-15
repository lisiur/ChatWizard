import { invoke } from "@tauri-apps/api";

export async function allChats() {
  return invoke<
    Array<{
      id: string;
      title: string;
    }>
  >("all_chats");
}

export async function readChat(chatId: string) {
  return invoke<
    Array<{
      id: string;
      message: {
        role: "system" | "user" | "assistant";
        content: string;
      };
    }>
  >("read_chat", { chatId });
}

export async function createChat(params?: { topic?: string; title?: string }) {
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

export function resetChat(chatId: string) {
  return invoke<void>("reset_chat", { chatId });
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
