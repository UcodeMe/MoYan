export type ClipboardItemType = 'text' | 'image'

export interface ClipboardItem {
  id: string
  type: ClipboardItemType
  content: string
  hash: string
  pinned: boolean
  createdAt: number
  size: number
  width?: number
  height?: number
}

export interface AppSettings {
  storageDir: string
  expirationDays: number
  clipboardHotkey: string
  screenshotHotkey: string
  launchAtLogin: boolean
}

export interface ScreenshotInit {
  dataUrl: string
  width: number
  height: number
}

export interface CompleteScreenshotOptions {
  dataUrl: string
  text: string
  saveToFile: boolean
}

export interface PastePayload {
  type: ClipboardItemType
  text?: string
  dataUrl?: string
}

export interface ElectronAPI {
  getSettings(): Promise<AppSettings>
  updateSettings(patch: Partial<AppSettings>): Promise<AppSettings>
  chooseDirectory(): Promise<string | null>
  getClipboardItems(): Promise<ClipboardItem[]>
  getClipboardImage(id: string, thumb?: boolean): Promise<string | null>
  copyText(text: string): Promise<void>
  copyImage(dataUrl: string): Promise<void>
  pasteToFocusedInput(payload: PastePayload): Promise<boolean>
  deleteClipboardItem(id: string): Promise<void>
  clearClipboard(): Promise<void>
  togglePin(id: string): Promise<void>
  onClipboardUpdated(cb: () => void): () => void
  onNavigate(cb: (route: 'clipboard' | 'settings') => void): () => void
  hideWindow(): void
  setKeepOpen(keep: boolean): void
  navigate(route: 'clipboard' | 'settings'): void
  takeScreenshot(): void
  getScreenshotInit(): Promise<ScreenshotInit | null>
  completeScreenshot(options: CompleteScreenshotOptions): Promise<string>
  closeScreenshot(): void
  getAppVersion(): Promise<string>
}
