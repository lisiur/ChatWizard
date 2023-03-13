import { clipboard } from '@tauri-apps/api'

export async function writeToClipboard(content: string) {
    await clipboard.writeText(content)
}