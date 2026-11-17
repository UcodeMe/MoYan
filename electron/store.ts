import fs from 'node:fs'
import path from 'node:path'
import { app } from 'electron'
import type { AppSettings, ClipboardItem } from '../shared/types'

function defaultSettings(): AppSettings {
  const base = app.getPath('documents') || app.getPath('home')
  return {
    storageDir: path.join(base, 'MoYan'),
    expirationDays: 30,
    clipboardHotkey: 'Shift+C',
    screenshotHotkey: 'F1',
    launchAtLogin: false
  }
}

function readJson<T>(file: string, fallback: T): T {
  try {
    if (!fs.existsSync(file)) return fallback
    return JSON.parse(fs.readFileSync(file, 'utf-8')) as T
  } catch {
    return fallback
  }
}

function writeJson(file: string, value: unknown): void {
  fs.mkdirSync(path.dirname(file), { recursive: true })
  fs.writeFileSync(file, JSON.stringify(value, null, 2), 'utf-8')
}

const settingsFile = (): string => path.join(app.getPath('userData'), 'settings.json')

export function loadSettings(): AppSettings {
  const stored = readJson<Partial<AppSettings>>(settingsFile(), {})
  return { ...defaultSettings(), ...stored }
}

export function saveSettings(settings: AppSettings): void {
  writeJson(settingsFile(), settings)
}

export const indexFile = (dir: string): string => path.join(dir, 'index.json')
export const imagesDir = (dir: string): string => path.join(dir, 'images')
export const thumbsDir = (dir: string): string => path.join(dir, 'thumbs')
export const screenshotsDir = (dir: string): string => path.join(dir, 'screenshots')

export function ensureStorage(dir: string): void {
  for (const d of [dir, imagesDir(dir), thumbsDir(dir), screenshotsDir(dir)]) {
    fs.mkdirSync(d, { recursive: true })
  }
  if (!fs.existsSync(indexFile(dir))) writeJson(indexFile(dir), [])
}

export function loadIndex(dir: string): ClipboardItem[] {
  ensureStorage(dir)
  const items = readJson<ClipboardItem[]>(indexFile(dir), [])
  return Array.isArray(items) ? items : []
}

export function saveIndex(dir: string, items: ClipboardItem[]): void {
  writeJson(indexFile(dir), items)
}

export function migrateStorage(from: string, to: string): void {
  const src = path.resolve(from)
  const dst = path.resolve(to)
  if (src === dst) {
    ensureStorage(to)
    return
  }
  if (!fs.existsSync(src)) {
    ensureStorage(to)
    return
  }
  ensureStorage(to)
  for (const name of ['images', 'thumbs', 'screenshots']) {
    const source = path.join(src, name)
    const target = path.join(dst, name)
    if (!fs.existsSync(source)) continue
    fs.mkdirSync(target, { recursive: true })
    for (const file of fs.readdirSync(source)) {
      const s = path.join(source, file)
      const d = path.join(target, file)
      if (!fs.existsSync(d)) fs.copyFileSync(s, d)
    }
  }
  const idx = indexFile(src)
  if (fs.existsSync(idx)) fs.copyFileSync(idx, indexFile(dst))
}
