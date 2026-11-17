import fs from 'node:fs'
import path from 'node:path'
import {
  app,
  BrowserWindow,
  clipboard,
  dialog,
  globalShortcut,
  ipcMain,
  Menu,
  nativeImage,
  screen,
  Tray
} from 'electron'
import type { AppSettings, CompleteScreenshotOptions, PastePayload } from '../shared/types'
import * as clipboardStore from './clipboard'
import * as screenshot from './screenshot'
import { pasteToFocusedInput } from './paste'
import {
  canPasteToRecordedWindow,
  pasteToRecordedWindow,
  startForegroundWatcher,
  stopForegroundWatcher
} from './native-helper'
import { getAppUrl, iconPath, preloadPath } from './window'
import { loadSettings, migrateStorage, saveSettings, screenshotsDir } from './store'

let settings: AppSettings = loadSettings()
let mainWindow: BrowserWindow | null = null
let tray: Tray | null = null
let keepOpen = false

function positionNearCursor(win: BrowserWindow): void {
  const point = screen.getCursorScreenPoint()
  const display = screen.getDisplayNearestPoint(point)
  const [width, height] = win.getSize()
  const x = Math.min(
    Math.max(point.x + 14, display.workArea.x),
    display.workArea.x + display.workArea.width - width
  )
  const y = Math.min(
    Math.max(point.y + 14, display.workArea.y),
    display.workArea.y + display.workArea.height - height
  )
  win.setPosition(x, y)
}

function createMainWindow(): void {
  mainWindow = new BrowserWindow({
    width: 440,
    height: 640,
    minWidth: 400,
    minHeight: 520,
    show: false,
    frame: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    resizable: true,
    backgroundColor: '#0b1020',
    webPreferences: {
      preload: preloadPath(),
      contextIsolation: true,
      nodeIntegration: false
    }
  })
  mainWindow.loadURL(getAppUrl('clipboard'))
  mainWindow.on('blur', () => {
    if (!keepOpen && mainWindow && !mainWindow.isDestroyed() && mainWindow.isVisible()) {
      mainWindow.hide()
    }
  })
  mainWindow.on('closed', () => {
    mainWindow = null
  })
}

function showMainWindow(route?: 'clipboard' | 'settings'): void {
  if (!mainWindow || mainWindow.isDestroyed()) createMainWindow()
  if (!mainWindow) return
  if (route) mainWindow.webContents.send('navigate', route)
  positionNearCursor(mainWindow)
  mainWindow.show()
  mainWindow.focus()
}

function toggleMainWindow(): void {
  if (screenshot.isEditorOpen()) return
  if (mainWindow && !mainWindow.isDestroyed() && mainWindow.isVisible()) {
    mainWindow.hide()
    return
  }
  showMainWindow()
}

function registerShortcuts(): void {
  globalShortcut.unregisterAll()
  if (!globalShortcut.register(settings.clipboardHotkey, toggleMainWindow)) {
    console.warn('剪贴板快捷键注册失败:', settings.clipboardHotkey)
  }
  if (!globalShortcut.register(settings.screenshotHotkey, () => {
    screenshot.captureScreen().catch((error) => console.error('截图失败', error))
  })) {
    console.warn('截图快捷键注册失败:', settings.screenshotHotkey)
  }
}

function createTray(): void {
  const image = nativeImage.createFromPath(iconPath('tray.png'))
  tray = new Tray(image.isEmpty() ? nativeImage.createEmpty() : image)
  tray.setToolTip('MoYan')
  tray.setContextMenu(Menu.buildFromTemplate([
    { label: '打开剪贴板', click: () => showMainWindow('clipboard') },
    { label: '截屏', click: () => screenshot.captureScreen().catch(console.error) },
    { label: '设置', click: () => showMainWindow('settings') },
    { type: 'separator' },
    { label: '退出 MoYan', click: () => app.quit() }
  ]))
  tray.on('click', () => showMainWindow())
}

function registerIpc(): void {
  ipcMain.handle('settings:get', () => settings)

  ipcMain.handle('settings:update', (_event, patch: Partial<AppSettings>) => {
    const previousDir = settings.storageDir
    settings = { ...settings, ...patch }
    if (settings.storageDir !== previousDir) {
      migrateStorage(previousDir, settings.storageDir)
    }
    saveSettings(settings)
    clipboardStore.updateSettings(settings)
    registerShortcuts()
    app.setLoginItemSettings({ openAtLogin: settings.launchAtLogin })
    return settings
  })

  ipcMain.handle('dialog:directory', async () => {
    const result = await dialog.showOpenDialog({
      properties: ['openDirectory', 'createDirectory'],
      title: '选择剪贴板存储目录'
    })
    return result.canceled || result.filePaths.length === 0 ? null : result.filePaths[0]
  })

  ipcMain.handle('clipboard:items', () => clipboardStore.getItems())
  ipcMain.handle('clipboard:image', (_event, id: string, thumb = true) =>
    clipboardStore.getImageDataUrl(id, thumb)
  )

  ipcMain.handle('clipboard:copyText', (_event, text: string) => {
    clipboard.writeText(text)
    clipboardStore.suppressCapture()
  })
  ipcMain.handle('clipboard:copyImage', (_event, dataUrl: string) => {
    clipboard.writeImage(nativeImage.createFromDataURL(dataUrl))
    clipboardStore.suppressCapture()
  })
  ipcMain.handle('clipboard:delete', (_event, id: string) => clipboardStore.deleteItem(id))
  ipcMain.handle('clipboard:clear', () => clipboardStore.clearItems())
  ipcMain.handle('clipboard:pin', (_event, id: string) => clipboardStore.togglePin(id))

  ipcMain.handle('clipboard:pasteToFocus', async (_event, payload: PastePayload) => {
    if (payload.type === 'text' && payload.text) {
      clipboard.writeText(payload.text)
    } else if (payload.type === 'image' && payload.dataUrl) {
      clipboard.writeImage(nativeImage.createFromDataURL(payload.dataUrl))
    }
    clipboardStore.suppressCapture()
    mainWindow?.hide()
    await new Promise((resolve) => setTimeout(resolve, 180))
    if (canPasteToRecordedWindow()) return pasteToRecordedWindow()
    return pasteToFocusedInput()
  })

  ipcMain.on('window:hide', () => mainWindow?.hide())
  ipcMain.on('window:keepOpen', (_event, keep: boolean) => {
    keepOpen = keep
  })
  ipcMain.on('navigate', (_event, route: 'clipboard' | 'settings') => {
    mainWindow?.webContents.send('navigate', route)
  })

  ipcMain.on('screenshot:trigger', () => {
    screenshot.captureScreen().catch((error) => console.error('截图失败', error))
  })
  ipcMain.handle('screenshot:init', () => screenshot.getPendingInit())
  ipcMain.handle('screenshot:complete', async (_event, options: CompleteScreenshotOptions) => {
    const dir = screenshotsDir(settings.storageDir)
    fs.mkdirSync(dir, { recursive: true })
    const stamp = new Date().toISOString().replace(/[:.]/g, '-')
    const file = path.join(dir, `screenshot-${stamp}.png`)
    const base64 = options.dataUrl.replace(/^data:image\/\w+;base64,/, '')
    fs.writeFileSync(file, Buffer.from(base64, 'base64'))
    if (options.saveToFile) {
      const result = await dialog.showSaveDialog({
        title: '保存截图',
        defaultPath: path.basename(file),
        filters: [{ name: 'PNG 图片', extensions: ['png'] }]
      })
      if (!result.canceled && result.filePath) fs.copyFileSync(file, result.filePath)
    }
    clipboard.writeImage(nativeImage.createFromDataURL(options.dataUrl))
    clipboardStore.suppressCapture()
    screenshot.closeEditor()
    return file
  })
  ipcMain.on('screenshot:close', () => screenshot.closeEditor())

  ipcMain.handle('app:version', () => app.getVersion())
}

function bootstrap(): void {
  const gotLock = app.requestSingleInstanceLock()
  if (!gotLock) {
    app.quit()
    return
  }
  app.on('second-instance', () => showMainWindow())

  app.whenReady().then(() => {
    clipboardStore.initClipboard(settings)
    createMainWindow()
    createTray()
    registerShortcuts()
    registerIpc()
    void startForegroundWatcher()
    showMainWindow()
    const cleanup = setInterval(() => clipboardStore.cleanupExpired(), 60 * 60 * 1000)
    cleanup.unref()

    app.on('activate', () => showMainWindow())
  })

  app.on('window-all-closed', () => {
    // 托盘常驻应用：所有窗口关闭后不退出
  })
  app.on('before-quit', () => {
    globalShortcut.unregisterAll()
    clipboardStore.stopClipboard()
    stopForegroundWatcher()
  })
}

bootstrap()
