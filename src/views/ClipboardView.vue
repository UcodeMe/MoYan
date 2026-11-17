<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'
import type { ClipboardItem } from '../../shared/types'
import AppIcon from '../components/AppIcon.vue'
import logoUrl from '../assets/logo.svg?url'

type Tab = 'all' | 'text' | 'image'

const items = ref<ClipboardItem[]>([])
const thumbs = ref<Record<string, string>>({})
const query = ref('')
const activeTab = ref<Tab>('all')
const toast = ref('')
const searchRef = ref<HTMLInputElement | null>(null)
let toastTimer: number | undefined
let offUpdate: (() => void) | undefined

const tabs: { key: Tab; label: string }[] = [
  { key: 'all', label: '全部' },
  { key: 'text', label: '文本' },
  { key: 'image', label: '图片' }
]

const filtered = computed(() => {
  const q = query.value.trim().toLowerCase()
  return items.value.filter((item) => {
    if (activeTab.value !== 'all' && item.type !== activeTab.value) return false
    if (!q) return true
    if (item.type === 'text') return item.content.toLowerCase().includes(q)
    return `图片 ${item.width ?? 0}×${item.height ?? 0}`.toLowerCase().includes(q)
  })
})

function showToast(message: string): void {
  toast.value = message
  if (toastTimer) window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => (toast.value = ''), 1600)
}

async function loadItems(): Promise<void> {
  items.value = await window.electronAPI.getClipboardItems()
  for (const item of items.value) {
    if (item.type === 'image' && !thumbs.value[item.id]) {
      const url = await window.electronAPI.getClipboardImage(item.id, true)
      if (url) thumbs.value[item.id] = url
    }
  }
}

async function copyItem(item: ClipboardItem): Promise<void> {
  try {
    if (item.type === 'text') {
      await window.electronAPI.copyText(item.content)
    } else {
      const full = await window.electronAPI.getClipboardImage(item.id, false)
      if (!full) throw new Error('图片文件不存在')
      await window.electronAPI.copyImage(full)
    }
    showToast('已复制到剪贴板')
  } catch (error) {
    showToast('复制失败')
    console.error(error)
  }
}

let clickTimer: number | undefined

function scheduleCopy(item: ClipboardItem): void {
  if (clickTimer) return
  clickTimer = window.setTimeout(() => {
    clickTimer = undefined
    void copyItem(item)
  }, 220)
}

async function pasteItem(item: ClipboardItem): Promise<void> {
  if (clickTimer) {
    window.clearTimeout(clickTimer)
    clickTimer = undefined
  }
  try {
    let dataUrl: string | undefined
    if (item.type === 'image') {
      dataUrl = (await window.electronAPI.getClipboardImage(item.id, false)) ?? undefined
      if (!dataUrl) throw new Error('图片文件不存在')
    }
    const pasted = await window.electronAPI.pasteToFocusedInput({
      type: item.type,
      text: item.type === 'text' ? item.content : undefined,
      dataUrl
    })
    showToast(pasted ? '已粘贴到当前输入框' : '已复制（当前焦点不是输入框）')
  } catch (error) {
    showToast('粘贴失败')
    console.error(error)
  }
}

async function pinItem(event: MouseEvent, item: ClipboardItem): Promise<void> {
  event.stopPropagation()
  await window.electronAPI.togglePin(item.id)
  await loadItems()
}

async function removeItem(event: MouseEvent, item: ClipboardItem): Promise<void> {
  event.stopPropagation()
  await window.electronAPI.deleteClipboardItem(item.id)
  delete thumbs.value[item.id]
  await loadItems()
  showToast('已删除')
}

async function clearAll(): Promise<void> {
  if (!window.confirm('确定清空剪贴板记录吗？已固定的条目会保留。')) return
  await window.electronAPI.clearClipboard()
  thumbs.value = {}
  await loadItems()
  showToast('已清空')
}

function formatTime(timestamp: number): string {
  const diff = Date.now() - timestamp
  if (diff < 60_000) return '刚刚'
  if (diff < 3_600_000) return `${Math.floor(diff / 60_000)} 分钟前`
  if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)} 小时前`
  const date = new Date(timestamp)
  return `${date.getMonth() + 1}月${date.getDate()}日`
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}

function onKeydown(event: KeyboardEvent): void {
  if (event.key === 'Escape') {
    window.electronAPI.hideWindow()
  }
}

function takeScreenshot(): void {
  window.electronAPI.takeScreenshot()
}

function openSettings(): void {
  window.location.hash = '#/settings'
}

onMounted(async () => {
  await loadItems()
  offUpdate = window.electronAPI.onClipboardUpdated(() => {
    void loadItems()
  })
  window.addEventListener('keydown', onKeydown)
  await nextTick()
  searchRef.value?.focus()
})

onUnmounted(() => {
  offUpdate?.()
  window.removeEventListener('keydown', onKeydown)
  if (toastTimer) window.clearTimeout(toastTimer)
  if (clickTimer) window.clearTimeout(clickTimer)
})
</script>

<template>
  <div class="clipboard-app">
    <header class="app-header">
      <img class="logo" :src="logoUrl" alt="MoYan" />
      <div class="brand">
        <strong>MoYan</strong>
        <span>剪贴板</span>
      </div>
      <div class="header-actions">
        <button class="btn-icon" title="截屏 (F1)" @click="takeScreenshot">
          <AppIcon name="camera" />
        </button>
        <button class="btn-icon" title="设置" @click="openSettings">
          <AppIcon name="settings" />
        </button>
      </div>
    </header>

    <div class="search-row">
      <AppIcon name="search" />
      <input ref="searchRef" v-model="query" class="search-input" placeholder="搜索剪贴板内容..." />
      <button class="clear-btn" title="清空" @click="clearAll">
        <AppIcon name="trash" />
      </button>
    </div>

    <div class="tabs">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="tab"
        :class="{ active: activeTab === tab.key }"
        @click="activeTab = tab.key"
      >
        {{ tab.label }}
        <span class="tab-count">
          {{ tab.key === 'all' ? items.length : items.filter((i) => i.type === tab.key).length }}
        </span>
      </button>
    </div>

    <div class="list">
      <div
        v-for="item in filtered"
        :key="item.id"
        class="item"
        title="单击复制，双击粘贴到当前输入框"
        @click="scheduleCopy(item)"
        @dblclick="pasteItem(item)"
      >
        <div class="item-icon" :class="item.type">
          <AppIcon :name="item.type === 'image' ? 'image' : 'text'" />
        </div>
        <div class="item-body">
          <div v-if="item.type === 'text'" class="item-text">{{ item.content }}</div>
          <div v-else class="item-image">
            <img v-if="thumbs[item.id]" :src="thumbs[item.id]" alt="图片缩略图" />
            <span v-else>图片加载中...</span>
          </div>
          <div class="item-meta">
            <span>{{ formatTime(item.createdAt) }}</span>
            <span class="dot">·</span>
            <span>{{ item.type === 'image' ? `${item.width}×${item.height} · ` : '' }}{{ formatSize(item.size) }}</span>
            <span v-if="item.pinned" class="pinned"><AppIcon name="pin" /> 已固定</span>
          </div>
        </div>
        <div class="item-actions">
          <button
            class="mini-btn"
            :class="{ on: item.pinned }"
            title="固定"
            @click="pinItem($event, item)"
            @dblclick.stop
          >
            <AppIcon name="pin" />
          </button>
          <button
            class="mini-btn danger"
            title="删除"
            @click="removeItem($event, item)"
            @dblclick.stop
          >
            <AppIcon name="trash" />
          </button>
        </div>
      </div>

      <div v-if="filtered.length === 0" class="empty">
        <AppIcon name="sparkles" />
        <p>{{ query ? '没有匹配的内容' : '剪贴板还是空的' }}</p>
        <span>{{ query ? '换个关键词试试' : '复制任意文本或图片，会自动出现在这里' }}</span>
      </div>
    </div>

    <footer class="hint">
      <span><kbd>Shift</kbd> + <kbd>C</kbd> 打开</span>
      <span><kbd>F1</kbd> 截屏</span>
      <span>单击复制 · 双击粘贴到输入框</span>
      <span><kbd>Esc</kbd> 关闭</span>
    </footer>

    <div v-if="toast" class="toast">{{ toast }}</div>
  </div>
</template>

<style scoped>
.clipboard-app {
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
  padding: 0 14px;
  border-bottom: 1px solid var(--border);
  -webkit-app-region: drag;
}

.logo {
  width: 26px;
  height: 26px;
}

.brand {
  display: flex;
  align-items: baseline;
  gap: 7px;
}

.brand strong {
  font-size: 15px;
  letter-spacing: 0.4px;
  background: linear-gradient(90deg, #7dd3fc, #a5b4fc);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.brand span {
  font-size: 11px;
  color: var(--faint);
}

.header-actions {
  margin-left: auto;
  display: flex;
  gap: 4px;
  -webkit-app-region: no-drag;
}

.search-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 12px 14px 8px;
  padding: 0 10px;
  height: 34px;
  border: 1px solid var(--border);
  border-radius: 9px;
  background: rgba(4, 8, 18, 0.55);
  color: var(--muted);
}

.search-row:focus-within {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(56, 189, 248, 0.1);
}

.search-input {
  flex: 1;
  min-width: 0;
  border: none;
  outline: none;
  background: transparent;
  color: var(--text);
  font-size: 12px;
}

.clear-btn {
  display: inline-flex;
  padding: 3px;
  border: none;
  background: transparent;
  color: var(--muted);
}

.clear-btn:hover {
  color: var(--danger);
}

.tabs {
  display: flex;
  gap: 4px;
  padding: 0 14px 8px;
  border-bottom: 1px solid var(--border);
}

.tab {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 5px 10px;
  border: none;
  border-radius: 7px;
  background: transparent;
  color: var(--muted);
  font-size: 12px;
}

.tab:hover {
  background: rgba(129, 168, 230, 0.08);
}

.tab.active {
  background: rgba(56, 189, 248, 0.14);
  color: #7dd3fc;
}

.tab-count {
  font-size: 10px;
  color: var(--faint);
}

.list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.item {
  display: flex;
  gap: 10px;
  padding: 9px;
  margin-bottom: 6px;
  border: 1px solid transparent;
  border-radius: 10px;
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}

.item:hover {
  background: var(--panel);
  border-color: var(--border);
}

.item-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  flex: none;
  border-radius: 8px;
  background: rgba(56, 189, 248, 0.12);
  color: var(--accent);
}

.item-icon.image {
  background: rgba(129, 140, 248, 0.14);
  color: var(--accent-2);
}

.item-body {
  flex: 1;
  min-width: 0;
}

.item-text {
  font-size: 12px;
  line-height: 1.5;
  color: var(--text);
  white-space: pre-wrap;
  word-break: break-all;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.item-image {
  max-height: 92px;
  overflow: hidden;
  border-radius: 6px;
  color: var(--faint);
  font-size: 11px;
}

.item-image img {
  display: block;
  max-width: 100%;
  max-height: 92px;
  object-fit: contain;
  border-radius: 6px;
}

.item-meta {
  display: flex;
  align-items: center;
  gap: 5px;
  margin-top: 5px;
  font-size: 10px;
  color: var(--faint);
}

.dot {
  opacity: 0.6;
}

.pinned {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  color: #fbbf24;
}

.pinned :deep(.app-icon) {
  width: 11px;
  height: 11px;
}

.item-actions {
  display: flex;
  flex-direction: column;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}

.item:hover .item-actions {
  opacity: 1;
}

.mini-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  padding: 0;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--muted);
}

.mini-btn :deep(.app-icon) {
  width: 14px;
  height: 14px;
}

.mini-btn:hover,
.mini-btn.on {
  background: rgba(129, 168, 230, 0.14);
  color: #fbbf24;
}

.mini-btn.danger:hover {
  color: var(--danger);
}

.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 60px 20px;
  color: var(--faint);
  text-align: center;
}

.empty :deep(.app-icon) {
  width: 34px;
  height: 34px;
  opacity: 0.6;
}

.empty p {
  margin: 4px 0 0;
  color: var(--muted);
  font-size: 13px;
}

.empty span {
  font-size: 11px;
}

.hint {
  display: flex;
  justify-content: center;
  gap: 16px;
  padding: 9px;
  border-top: 1px solid var(--border);
  color: var(--faint);
  font-size: 10px;
}

kbd {
  padding: 1px 5px;
  border: 1px solid var(--border-strong);
  border-radius: 4px;
  background: var(--panel-2);
  color: var(--muted);
  font-family: inherit;
  font-size: 10px;
}
</style>
