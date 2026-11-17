import path from 'node:path'
import { app } from 'electron'

export function getAppUrl(hash = ''): string {
  const devUrl = process.env.VITE_DEV_SERVER_URL || 'http://localhost:5173'
  const base = app.isPackaged
    ? `file://${path.join(app.getAppPath(), 'dist', 'index.html')}`
    : devUrl
  return hash ? `${base}#/${hash}` : base
}

export function preloadPath(): string {
  return path.join(app.getAppPath(), 'dist-electron', 'electron', 'preload.js')
}

export function iconPath(name = 'tray.png'): string {
  return app.isPackaged
    ? path.join(process.resourcesPath, 'icons', name)
    : path.join(app.getAppPath(), 'build', 'icons', name)
}
