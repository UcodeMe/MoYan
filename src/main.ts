import { createApp } from 'vue'
import App from './App.vue'
import { electronAPI } from './bridge'
import './styles/main.css'

// Tauri 桥接：挂载到 window.electronAPI，保持与 Electron preload 相同的全局接口
window.electronAPI = electronAPI
createApp(App).mount('#app')
