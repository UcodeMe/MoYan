import { BrowserWindow, desktopCapturer, screen } from 'electron'
import type { ScreenshotInit } from '../shared/types'
import { getAppUrl, preloadPath } from './window'

let editorWindow: BrowserWindow | null = null
let pending: ScreenshotInit | null = null

export function isEditorOpen(): boolean {
  return !!editorWindow && !editorWindow.isDestroyed()
}

export function getPendingInit(): ScreenshotInit | null {
  return pending
}

export async function captureScreen(): Promise<void> {
  const display = screen.getDisplayNearestPoint(screen.getCursorScreenPoint())
  const { width, height } = display.size
  const scale = display.scaleFactor
  const sources = await desktopCapturer.getSources({
    types: ['screen'],
    thumbnailSize: {
      width: Math.max(1, Math.round(width * scale)),
      height: Math.max(1, Math.round(height * scale))
    }
  })
  if (sources.length === 0) throw new Error('未获取到屏幕画面')
  const image = sources[0].thumbnail
  const size = image.getSize()
  pending = {
    dataUrl: image.toDataURL(),
    width: size.width,
    height: size.height
  }
  openEditor(display.bounds)
}

function openEditor(bounds: Electron.Rectangle): void {
  if (isEditorOpen()) {
    editorWindow!.focus()
    return
  }
  editorWindow = new BrowserWindow({
    x: bounds.x,
    y: bounds.y,
    width: bounds.width,
    height: bounds.height,
    frame: false,
    fullscreen: true,
    alwaysOnTop: true,
    skipTaskbar: true,
    resizable: false,
    backgroundColor: '#070b16',
    webPreferences: {
      preload: preloadPath(),
      contextIsolation: true,
      nodeIntegration: false
    }
  })
  editorWindow.loadURL(getAppUrl('screenshot'))
  editorWindow.on('closed', () => {
    editorWindow = null
    pending = null
  })
}

export function closeEditor(): void {
  if (isEditorOpen()) editorWindow!.close()
  else {
    editorWindow = null
    pending = null
  }
}
