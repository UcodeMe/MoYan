<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import ClipboardView from './views/ClipboardView.vue'
import SettingsView from './views/SettingsView.vue'
import ScreenshotView from './views/ScreenshotView.vue'

type Route = 'clipboard' | 'settings' | 'screenshot'

// Tauri：screenshot 窗口与主窗口加载同一前端，按窗口 label 判定初始视图
const initialRoute: Route = getCurrentWindow().label === 'screenshot' ? 'screenshot' : 'clipboard'
const route = ref<Route>(initialRoute)

function syncRoute(): void {
  const value = window.location.hash.replace(/^#\/?/, '')
  route.value = value === 'settings' || value === 'screenshot' ? value : 'clipboard'
}

function onNavigate(target: Route): void {
  route.value = target
  window.location.hash = `#/${target}`
}

function onIpcNavigate(target: Route): void {
  onNavigate(target)
}

onMounted(() => {
  // 仅当 URL 带有显式 hash 时才同步路由（screenshot 窗口加载时无 hash，
  // 必须保留按窗口 label 判定的初始路由，否则会被覆盖回 clipboard）
  if (window.location.hash.replace(/^#\/?/, '')) {
    syncRoute()
  }
  window.addEventListener('hashchange', syncRoute)
  window.electronAPI.onNavigate(onIpcNavigate)
})

onUnmounted(() => {
  window.removeEventListener('hashchange', syncRoute)
})

watch(route, (value) => {
  window.electronAPI.setKeepOpen(value === 'settings')
})
</script>

<template>
  <ClipboardView v-if="route === 'clipboard'" />
  <SettingsView v-else-if="route === 'settings'" />
  <ScreenshotView v-else />
</template>
