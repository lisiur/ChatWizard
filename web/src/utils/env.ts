export const isTauri = "__TAURI_IPC__" in window;
export const isWeb = !isTauri;
