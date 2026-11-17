import fs from 'node:fs'
import path from 'node:path'
import crypto from 'node:crypto'
import { BrowserWindow, clipboard, nativeImage } from 'electron'
import type { AppSettings, ClipboardItem } from '../shared/types'
import { ensureStorage, imagesDir, loadIndex, saveIndex, thumbsDir } from './store'

let settings: AppSettings
let items: ClipboardItem[] = []
let timer: NodeJS.Timeout | null = null
let lastTextHash = ''
let lastImageHash = ''
let suppressUntil = 0
const MAX_ITEMS = 1000

function digest(data: string | Buffer): string {
  return crypto.createHash('sha256').update(data).digest('hex')
}

function readTextSafe(): string {
  try {
    return clipboard.readText()
  } catch {
    return ''
  }
}

function readImageSafe(): ReturnType<typeof nativeImage.createEmpty> | null {
  try {
    const img = clipboard.readImage()
    return img.isEmpty() ? null : img
  } catch {
    return null
  }
}

function notify(): void {
  for (const win of BrowserWindow.getAllWindows()) {
    if (!win.isDestroyed()) win.webContents.send('clipboard:updated')
  }
}

function persist(): void {
  saveIndex(settings.storageDir, items)
  notify()
}

function removeFiles(item: ClipboardItem): void {
  for (const dir of [imagesDir(settings.storageDir), thumbsDir(settings.storageDir)]) {
    const file = path.join(dir, `${item.id}.png`)
    try {
      if (fs.existsSync(file)) fs.unlinkSync(file)
    } catch (error) {
      console.warn('删除剪贴板文件失败', file, error)
    }
  }
}

function trim(): void {
  const removable = items.filter((item) => !item.pinned)
  while (items.length > MAX_ITEMS && removable.length > 0) {
    const victim = removable.shift()!
    items = items.filter((item) => item.id !== victim.id)
    removeFiles(victim)
  }
}

function addText(content: string): void {
  items.unshift({
    id: crypto.randomUUID(),
    type: 'text',
    content,
    hash: digest(content),
    pinned: false,
    createdAt: Date.now(),
    size: Buffer.byteLength(content)
  })
  trim()
  persist()
}

function addImage(img: Electron.NativeImage, png: Buffer): void {
  const id = crypto.randomUUID()
  const size = img.getSize()
  try {
    fs.writeFileSync(path.join(imagesDir(settings.storageDir), `${id}.png`), png)
    let thumb = png
    try {
      thumb = img.resize({ width: 280 }).toPNG()
    } catch {
      // 缩略图生成失败时回退到原图
    }
    fs.writeFileSync(path.join(thumbsDir(settings.storageDir), `${id}.png`), thumb)
  } catch (error) {
    console.error('保存剪贴板图片失败', error)
    return
  }
  items.unshift({
    id,
    type: 'image',
    content: '',
    hash: digest(png),
    pinned: false,
    createdAt: Date.now(),
    size: png.length,
    width: size.width,
    height: size.height
  })
  trim()
  persist()
}

function syncHashes(): void {
  const text = readTextSafe().trim()
  lastTextHash = text ? digest(text) : ''
  const img = readImageSafe()
  lastImageHash = img ? digest(img.toPNG()) : ''
}

function poll(): void {
  try {
    if (Date.now() < suppressUntil) {
      syncHashes()
      return
    }
    const text = readTextSafe().trim()
    if (text) {
      const h = digest(text)
      if (h !== lastTextHash) {
        lastTextHash = h
        addText(text)
      }
    } else {
      lastTextHash = ''
    }
    const img = readImageSafe()
    if (img) {
      const png = img.toPNG()
      const h = digest(png)
      if (h !== lastImageHash) {
        lastImageHash = h
        addImage(img, png)
      }
    } else {
      lastImageHash = ''
    }
  } catch (error) {
    console.error('剪贴板轮询失败', error)
  }
}

export function cleanupExpired(): void {
  const maxAge = settings.expirationDays > 0 ? settings.expirationDays * 24 * 60 * 60 * 1000 : Infinity
  const now = Date.now()
  const keep = items.filter((item) => item.pinned || now - item.createdAt <= maxAge)
  if (keep.length === items.length) return
  const removed = items.filter((item) => !keep.includes(item))
  removed.forEach(removeFiles)
  items = keep
  persist()
}

export function initClipboard(current: AppSettings): void {
  settings = current
  ensureStorage(settings.storageDir)
  items = loadIndex(settings.storageDir)
  syncHashes()
  cleanupExpired()
  startMonitoring()
}

export function stopClipboard(): void {
  if (timer) clearInterval(timer)
  timer = null
}

export function startMonitoring(): void {
  stopClipboard()
  timer = setInterval(poll, 700)
}

export function updateSettings(next: AppSettings): void {
  settings = next
  ensureStorage(settings.storageDir)
  items = loadIndex(settings.storageDir)
  cleanupExpired()
  syncHashes()
}

export function getItems(): ClipboardItem[] {
  return [...items].sort((a, b) => {
    if (a.pinned !== b.pinned) return a.pinned ? -1 : 1
    return b.createdAt - a.createdAt
  })
}

export function getImageDataUrl(id: string, thumb = true): string | null {
  const item = items.find((entry) => entry.id === id)
  if (!item || item.type !== 'image') return null
  const dir = thumb ? thumbsDir(settings.storageDir) : imagesDir(settings.storageDir)
  const file = path.join(dir, `${id}.png`)
  try {
    if (!fs.existsSync(file)) return null
    return `data:image/png;base64,${fs.readFileSync(file).toString('base64')}`
  } catch {
    return null
  }
}

export function deleteItem(id: string): void {
  const target = items.find((item) => item.id === id)
  if (!target) return
  items = items.filter((item) => item.id !== id)
  removeFiles(target)
  persist()
}

export function clearItems(): void {
  const removed = items.filter((item) => !item.pinned)
  removed.forEach(removeFiles)
  items = items.filter((item) => item.pinned)
  persist()
}

export function togglePin(id: string): void {
  const target = items.find((item) => item.id === id)
  if (!target) return
  target.pinned = !target.pinned
  persist()
}

export function suppressCapture(): void {
  suppressUntil = Date.now() + 1800
  syncHashes()
}
