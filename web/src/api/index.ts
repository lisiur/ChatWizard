import { invoke, execCommand } from "../utils/api";

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
  finished: boolean;
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
  return execCommand<ChatIndex>("get_chat", { id });
}

export async function allChats() {
  return execCommand<Array<ChatIndex>>("all_chats");
}

export async function loadChatLogByCursor(params: {
  chatId: string;
  cursor?: string;
  size: number;
}) {
  const res = await execCommand<{
    records: Array<ChatLog>;
    nextCursor: string | null;
  }>("load_chat_log_by_cursor", params);
  return res;
}

export async function updateChat(payload: ChatUpdatePayload) {
  return execCommand<void>("update_chat", { payload });
}

export async function newChat(params?: { promptId?: string; title?: string }) {
  return execCommand<string>("new_chat", params);
}

export async function deleteChat(chatId: string) {
  return execCommand<void>("delete_chat", { chatId });
}

export async function allNonStickChats() {
  return execCommand<Array<ChatIndex>>("all_non_stick_chats");
}

export async function allStickChats() {
  return execCommand<Array<ChatIndex>>("all_stick_chats");
}

export async function allArchiveChats() {
  return execCommand<Array<ChatIndex>>("all_archive_chats");
}

export async function setChatArchive(chatId: string) {
  return execCommand<void>("set_chat_archive", { chatId });
}

export async function setChatStick(chatId: string, stick: boolean) {
  return execCommand<void>("set_chat_stick", { chatId, stick });
}

export async function moveStickChat(from: string, to: string) {
  return execCommand<void>("move_stick_chat", { from, to });
}

export async function moveNonStickChat(from: string, to: string) {
  return execCommand<void>("move_non_stick_chat", { from, to });
}

export async function deleteChatLog(logId: string) {
  return execCommand<void>("delete_chat_log", { logId });
}

export function sendMessage(chatId: string, message: string) {
  return execCommand<[string, string]>("send_message", { chatId, message });
}

export function resendMessage(messageId: string) {
  return execCommand<[string, string]>("resend_message", { messageId });
}

export function getChatModels() {
  return execCommand<Array<ChatModel>>("get_chat_models");
}

export function allPrompts() {
  return execCommand<Array<PromptIndex>>("all_prompts");
}

export function createPrompt(prompt: { name: string; content: string }) {
  return execCommand<string>("create_prompt", prompt);
}

export function updatePrompt(payload: PromptUpdatePayload) {
  return execCommand("update_prompt", { payload });
}

export function deletePrompt(id: string) {
  return execCommand("delete_prompt", { id });
}

export function loadPrompt(id: string) {
  return execCommand<PromptData>("load_prompt", { id });
}

export function getPromptSources() {
  return execCommand<Array<PromptMarketSource>>("get_prompt_sources");
}

export function getPromptSourcePrompts(sourceId: string) {
  return execCommand<Array<MarketPrompt>>("get_prompt_source_prompts", {
    sourceId,
  });
}

export function installMarketPrompt(prompt: MarketPrompt) {
  return execCommand<string>("install_market_prompt", { ...prompt });
}

export function installMarketPromptAndCreateChat(prompt: MarketPrompt) {
  return execCommand<string>("install_market_prompt_and_create_chat", {
    ...prompt,
  });
}

export function getSettings() {
  return execCommand<Settings>("get_settings");
}

export function updateSettings(payload: Settings) {
  return execCommand<void>("update_settings", { payload });
}

export function getTheme() {
  return execCommand<Theme>("get_theme");
}

export function getLocale() {
  return execCommand<string>("get_locale");
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
