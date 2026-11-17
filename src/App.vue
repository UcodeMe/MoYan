<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import ClipboardView from './views/ClipboardView.vue'
import SettingsView from './views/SettingsView.vue'
import ScreenshotView from './views/ScreenshotView.vue'

type Route = 'clipboard' | 'settings' | 'screenshot'

const route = ref<Route>('clipboard')

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
  syncRoute()
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
