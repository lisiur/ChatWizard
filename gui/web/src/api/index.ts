import { invoke } from "@tauri-apps/api";

export async function createChat() {
    return invoke<string>("new_chat")
}

export function sendMessage(chatId: string, message: string) {
    return invoke<string>("send_message", { chatId, message })
}

export function resendMessage(chatId: string, messageId: string) {
    return invoke<void>("resend_message", { chatId, messageId })
}

export function resetChat(chatId: string) {
    return invoke<void>("reset_chat", { chatId })
} 

export function setApiKey(apiKey: string) {
    return invoke<void>("set_api_key", { apiKey })
}

export function checkApiKey() {
    return invoke<void>("has_api_key")
}

export function setProxy(proxy: string) {
    return invoke<void>("set_proxy", { proxy })
}

export function getProxy() {
    return invoke<string>("get_proxy")
}

export function hasApiKey() {
    return invoke<boolean>("has_api_key")
}

export function showMainWindow() {
    return invoke<void>("show_main_window")
}