import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { ElectronAPI } from '../shared/types'

// Tauri 桥接层：实现与原 Electron preload 完全一致的 ElectronAPI 接口，
// 前端 Vue 组件无需任何改动。
export const electronAPI: ElectronAPI = {
  getSettings: () => invoke('settings_get'),
  updateSettings: (patch) => invoke('settings_update', { patch }),
  chooseDirectory: () => invoke('dialog_directory'),
  getClipboardItems: () => invoke('clipboard_items'),
  getClipboardImage: (id, thumb = true) => invoke('clipboard_image', { id, thumb }),
  copyText: (text) => invoke('clipboard_copy_text', { text }),
  copyImage: (dataUrl) => invoke('clipboard_copy_image', { dataUrl }),
  pasteToFocusedInput: (payload) => invoke('clipboard_paste_to_focus', { payload }),
  deleteClipboardItem: (id) => invoke('clipboard_delete_item', { id }),
  clearClipboard: () => invoke('clipboard_clear'),
  togglePin: (id) => invoke('clipboard_pin', { id }),
  onClipboardUpdated: (cb) => {
    const ready = listen('clipboard:updated', () => cb())
    return () => {
      void ready.then((unlisten) => unlisten())
    }
  },
  onNavigate: (cb) => {
    const ready = listen<'clipboard' | 'settings'>('navigate', (event) => cb(event.payload))
    return () => {
      void ready.then((unlisten) => unlisten())
    }
  },
  hideWindow: () => invoke('window_hide'),
  setKeepOpen: (keep) => invoke('window_keep_open', { keep }),
  navigate: (route) => invoke('navigate', { route }),
  takeScreenshot: () => invoke('screenshot_trigger'),
  getScreenshotInit: () => invoke('screenshot_init'),
  completeScreenshot: (options) => invoke('screenshot_complete', { options }),
  closeScreenshot: () => invoke('screenshot_close'),
  getAppVersion: () => invoke('app_version')
}
