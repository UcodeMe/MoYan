<script setup lang="ts">
import { onMounted, onUnmounted, reactive, ref } from 'vue'
import type { AppSettings } from '../../shared/types'
import AppIcon from '../components/AppIcon.vue'
import logoUrl from '../assets/logo.svg?url'

type HotkeyField = 'clipboardHotkey' | 'screenshotHotkey'

const form = reactive<AppSettings>({
  storageDir: '',
  expirationDays: 30,
  clipboardHotkey: 'Shift+C',
  screenshotHotkey: 'F1',
  launchAtLogin: false
})

const recording = ref<HotkeyField | null>(null)
const toast = ref('')
const version = ref('')
let toastTimer: number | undefined

function showToast(message: string): void {
  toast.value = message
  if (toastTimer) window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => (toast.value = ''), 1600)
}

function normalizeKey(key: string): string {
  if (['Shift', 'Control', 'Alt', 'Meta'].includes(key)) return ''
  if (/^F\d{1,2}$/i.test(key)) return key.toUpperCase()
  if (/^[a-z]$/i.test(key)) return key.toUpperCase()
  const map: Record<string, string> = {
    ' ': 'Space',
    ArrowUp: 'Up',
    ArrowDown: 'Down',
    ArrowLeft: 'Left',
    ArrowRight: 'Right',
    Escape: 'Escape',
    Enter: 'Enter',
    Tab: 'Tab',
    Backspace: 'Backspace',
    Delete: 'Delete'
  }
  return map[key] || key
}

function onKeydown(event: KeyboardEvent): void {
  if (!recording.value) return
  event.preventDefault()
  event.stopPropagation()
  if (event.key === 'Escape') {
    recording.value = null
    return
  }
  const parts: string[] = []
  if (event.ctrlKey || event.metaKey) parts.push('CommandOrControl')
  if (event.altKey) parts.push('Alt')
  if (event.shiftKey) parts.push('Shift')
  const key = normalizeKey(event.key)
  if (!key) return
  parts.push(key)
  form[recording.value] = parts.join('+')
  recording.value = null
}

function startRecording(field: HotkeyField): void {
  recording.value = field
}

function displayHotkey(field: HotkeyField, value: string): string {
  if (recording.value === field) return '按下快捷键...'
  return value.replace(/CommandOrControl/g, 'Ctrl')
}

async function chooseDirectory(): Promise<void> {
  const dir = await window.electronAPI.chooseDirectory()
  if (dir) form.storageDir = dir
}

async function save(): Promise<void> {
  await window.electronAPI.updateSettings({ ...form })
  showToast('设置已保存')
}

function goBack(): void {
  window.location.hash = '#/clipboard'
}

onMounted(async () => {
  const settings = await window.electronAPI.getSettings()
  Object.assign(form, settings)
  version.value = await window.electronAPI.getAppVersion()
  window.addEventListener('keydown', onKeydown, true)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown, true)
  if (toastTimer) window.clearTimeout(toastTimer)
})
</script>

<template>
  <div class="settings-app">
    <header class="app-header">
      <button class="btn-icon" title="返回" @click="goBack">
        <AppIcon name="back" />
      </button>
      <img class="logo" :src="logoUrl" alt="MoYan" />
      <strong>设置</strong>
      <span class="version">v{{ version }}</span>
    </header>

    <div class="content">
      <section class="card">
        <div class="card-title"><AppIcon name="folder" /> 存储设置</div>
        <label class="field-label">数据存储位置</label>
        <div class="path-row">
          <input v-model="form.storageDir" class="input path-input" spellcheck="false" />
          <button class="btn" @click="chooseDirectory">选择</button>
        </div>
        <p class="tip">默认位于“文档/MoYan”。更换目录时会自动迁移已有数据。</p>

        <label class="field-label">数据过期时间</label>
        <div class="expire-row">
          <button
            v-for="days in [7, 30, 90, 0]"
            :key="days"
            class="chip"
            :class="{ active: form.expirationDays === days }"
            @click="form.expirationDays = days"
          >
            {{ days === 0 ? '永久' : `${days} 天` }}
          </button>
        </div>
        <p class="tip">超过期限的非固定记录会自动清理，默认为 30 天。</p>
      </section>

      <section class="card">
        <div class="card-title"><AppIcon name="keyboard" /> 快捷键</div>

        <label class="field-label">打开剪贴板</label>
        <button
          class="hotkey"
          :class="{ recording: recording === 'clipboardHotkey' }"
          @click="startRecording('clipboardHotkey')"
        >
          {{ displayHotkey('clipboardHotkey', form.clipboardHotkey) || '点击设置' }}
        </button>

        <label class="field-label">截屏</label>
        <button
          class="hotkey"
          :class="{ recording: recording === 'screenshotHotkey' }"
          @click="startRecording('screenshotHotkey')"
        >
          {{ displayHotkey('screenshotHotkey', form.screenshotHotkey) || '点击设置' }}
        </button>
        <p class="tip">点击输入框后按下组合键，Esc 取消录制。</p>
      </section>

      <section class="card">
        <div class="card-title"><AppIcon name="sparkles" /> 通用</div>
        <label class="switch-row">
          <span>开机自动启动</span>
          <input v-model="form.launchAtLogin" type="checkbox" />
          <span class="switch" />
        </label>
      </section>
    </div>

    <footer class="footer">
      <button class="btn btn-ghost" @click="goBack">取消</button>
      <button class="btn btn-primary" @click="save"><AppIcon name="check" /> 保存设置</button>
    </footer>

    <div v-if="toast" class="toast">{{ toast }}</div>
  </div>
</template>

<style scoped>
.settings-app {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: linear-gradient(160deg, #0d1428 0%, #0a0f1e 100%);
}

.app-header {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 52px;
  padding: 0 12px;
  border-bottom: 1px solid var(--border);
  -webkit-app-region: drag;
}

.app-header > * {
  -webkit-app-region: no-drag;
}

.logo {
  width: 24px;
  height: 24px;
}

.app-header strong {
  font-size: 15px;
}

.version {
  margin-left: auto;
  font-size: 10px;
  color: var(--faint);
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.card {
  padding: 14px;
  margin-bottom: 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: rgba(16, 26, 49, 0.6);
}

.card-title {
  display: flex;
  align-items: center;
  gap: 7px;
  margin-bottom: 14px;
  font-size: 13px;
  font-weight: 600;
  color: #a5d8ff;
}

.card-title :deep(.app-icon) {
  width: 16px;
  height: 16px;
}

.field-label {
  display: block;
  margin: 12px 0 6px;
  font-size: 11px;
  color: var(--muted);
}

.field-label:first-of-type {
  margin-top: 0;
}

.path-row {
  display: flex;
  gap: 8px;
}

.path-input {
  min-width: 0;
  font-size: 11px;
}

.tip {
  margin: 6px 0 0;
  font-size: 10px;
  line-height: 1.5;
  color: var(--faint);
}

.expire-row {
  display: flex;
  gap: 6px;
}

.chip {
  flex: 1;
  height: 30px;
  border: 1px solid var(--border);
  border-radius: 7px;
  background: rgba(7, 11, 22, 0.5);
  color: var(--muted);
  font-size: 11px;
}

.chip:hover {
  border-color: var(--accent);
}

.chip.active {
  border-color: transparent;
  background: linear-gradient(120deg, #0ea5e9, #6366f1);
  color: #fff;
}

.hotkey {
  display: block;
  width: 100%;
  height: 34px;
  padding: 0 12px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: rgba(7, 11, 22, 0.6);
  color: var(--text);
  text-align: left;
  font-size: 12px;
}

.hotkey:hover {
  border-color: var(--accent);
}

.hotkey.recording {
  border-color: #fbbf24;
  color: #fbbf24;
  box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.12);
}

.switch-row {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 12px;
  cursor: pointer;
}

.switch-row input {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.switch {
  position: relative;
  width: 36px;
  height: 20px;
  margin-left: auto;
  border-radius: 999px;
  background: var(--panel-3);
  transition: background 0.2s;
}

.switch::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #cbd5e1;
  transition: transform 0.2s, background 0.2s;
}

.switch-row input:checked + .switch {
  background: linear-gradient(120deg, #0ea5e9, #6366f1);
}

.switch-row input:checked + .switch::after {
  transform: translateX(16px);
  background: #fff;
}

.footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 10px 12px;
  border-top: 1px solid var(--border);
}
</style>
