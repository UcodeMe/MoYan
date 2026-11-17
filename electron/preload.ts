import { contextBridge, ipcRenderer } from 'electron'
import type { ElectronAPI } from '../shared/types'

const api: ElectronAPI = {
  getSettings: () => ipcRenderer.invoke('settings:get'),
  updateSettings: (patch) => ipcRenderer.invoke('settings:update', patch),
  chooseDirectory: () => ipcRenderer.invoke('dialog:directory'),
  getClipboardItems: () => ipcRenderer.invoke('clipboard:items'),
  getClipboardImage: (id, thumb = true) => ipcRenderer.invoke('clipboard:image', id, thumb),
  copyText: (text) => ipcRenderer.invoke('clipboard:copyText', text),
  copyImage: (dataUrl) => ipcRenderer.invoke('clipboard:copyImage', dataUrl),
  pasteToFocusedInput: (payload) => ipcRenderer.invoke('clipboard:pasteToFocus', payload),
  deleteClipboardItem: (id) => ipcRenderer.invoke('clipboard:delete', id),
  clearClipboard: () => ipcRenderer.invoke('clipboard:clear'),
  togglePin: (id) => ipcRenderer.invoke('clipboard:pin', id),
  onClipboardUpdated: (cb) => {
    const listener = () => cb()
    ipcRenderer.on('clipboard:updated', listener)
    return () => ipcRenderer.removeListener('clipboard:updated', listener)
  },
  onNavigate: (cb) => {
    const listener = (_event: Electron.IpcRendererEvent, route: 'clipboard' | 'settings') => cb(route)
    ipcRenderer.on('navigate', listener)
    return () => ipcRenderer.removeListener('navigate', listener)
  },
  hideWindow: () => ipcRenderer.send('window:hide'),
  setKeepOpen: (keep) => ipcRenderer.send('window:keepOpen', keep),
  navigate: (route) => ipcRenderer.send('navigate', route),
  takeScreenshot: () => ipcRenderer.send('screenshot:trigger'),
  getScreenshotInit: () => ipcRenderer.invoke('screenshot:init'),
  completeScreenshot: (options) => ipcRenderer.invoke('screenshot:complete', options),
  closeScreenshot: () => ipcRenderer.send('screenshot:close'),
  getAppVersion: () => ipcRenderer.invoke('app:version')
}

contextBridge.exposeInMainWorld('electronAPI', api)
