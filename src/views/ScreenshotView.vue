<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'
import AppIcon from '../components/AppIcon.vue'

type Point = [number, number]
type ShapeTool = 'rect' | 'ellipse' | 'arrow'
type DrawTool = 'pen' | ShapeTool
type Tool = DrawTool | 'crop' | 'text'

type Op =
  | { kind: 'pen'; points: Point[]; color: string; width: number }
  | { kind: ShapeTool; x1: number; y1: number; x2: number; y2: number; color: string; width: number }
  | { kind: 'text'; x: number; y: number; text: string; color: string; size: number }

const tools: { key: Tool; icon: string; label: string }[] = [
  { key: 'crop', icon: 'crop', label: '裁剪' },
  { key: 'pen', icon: 'pen', label: '画笔' },
  { key: 'rect', icon: 'rect', label: '矩形' },
  { key: 'ellipse', icon: 'circle', label: '椭圆' },
  { key: 'arrow', icon: 'arrow', label: '箭头' },
  { key: 'text', icon: 'text', label: '文字' }
]
const colors = ['#ff4d6d', '#fbbf24', '#22d3ee', '#4ade80', '#f8fafc']

const canvasRef = ref<HTMLCanvasElement | null>(null)
const wrapRef = ref<HTMLDivElement | null>(null)
const inlineInput = ref<HTMLInputElement | null>(null)

const tool = ref<Tool>('pen')
const color = ref('#ff4d6d')
const noteText = ref('')
const ops = ref<Op[]>([])
const draft = ref<Op | null>(null)
const inlinePos = ref<Point | null>(null)
const inlineText = ref('')
const strokeWidth = ref(4)
const toast = ref('')
const error = ref('')
let toastTimer: number | undefined

let baseCanvas: HTMLCanvasElement
let originalCanvas: HTMLCanvasElement
let imageW = 0
let imageH = 0
let scale = 1
let drawing = false
let cropStart: Point | null = null
let cropCurrent: Point | null = null

function cloneCanvas(source: HTMLCanvasElement): HTMLCanvasElement {
  const copy = document.createElement('canvas')
  copy.width = source.width
  copy.height = source.height
  copy.getContext('2d')!.drawImage(source, 0, 0)
  return copy
}

function showToast(message: string): void {
  toast.value = message
  if (toastTimer) window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => (toast.value = ''), 1500)
}

function layout(): void {
  const wrap = wrapRef.value
  const canvas = canvasRef.value
  if (!wrap || !canvas || !imageW || !imageH) return
  const availW = Math.max(120, wrap.clientWidth - 48)
  const availH = Math.max(120, wrap.clientHeight - 48)
  scale = Math.min(availW / imageW, availH / imageH, 1)
  canvas.width = Math.max(1, Math.floor(imageW * scale))
  canvas.height = Math.max(1, Math.floor(imageH * scale))
  render()
}

function drawOp(ctx: CanvasRenderingContext2D, op: Op): void {
  ctx.save()
  ctx.strokeStyle = op.color
  ctx.fillStyle = op.color
  ctx.lineCap = 'round'
  ctx.lineJoin = 'round'
  if (op.kind === 'text') {
    ctx.font = `600 ${op.size}px "PingFang SC", "Microsoft YaHei", sans-serif`
    ctx.textBaseline = 'top'
    ctx.shadowColor = 'rgba(0,0,0,.55)'
    ctx.shadowBlur = 4
    ctx.fillText(op.text, op.x, op.y)
  } else if (op.kind === 'pen') {
    ctx.lineWidth = op.width
    ctx.beginPath()
    op.points.forEach((p: Point, i: number) => (i === 0 ? ctx.moveTo(p[0], p[1]) : ctx.lineTo(p[0], p[1])))
    if (op.points.length === 1) ctx.lineTo(op.points[0][0] + 0.1, op.points[0][1] + 0.1)
    ctx.stroke()
  } else {
    ctx.lineWidth = op.width
    if (op.kind === 'rect') {
      ctx.strokeRect(op.x1, op.y1, op.x2 - op.x1, op.y2 - op.y1)
    } else if (op.kind === 'ellipse') {
      ctx.beginPath()
      ctx.ellipse(
        (op.x1 + op.x2) / 2,
        (op.y1 + op.y2) / 2,
        Math.abs(op.x2 - op.x1) / 2,
        Math.abs(op.y2 - op.y1) / 2,
        0,
        0,
        Math.PI * 2
      )
      ctx.stroke()
    } else {
      const angle = Math.atan2(op.y2 - op.y1, op.x2 - op.x1)
      const head = Math.max(14, op.width * 4)
      ctx.beginPath()
      ctx.moveTo(op.x1, op.y1)
      ctx.lineTo(op.x2, op.y2)
      ctx.stroke()
      ctx.beginPath()
      ctx.moveTo(op.x2, op.y2)
      ctx.lineTo(op.x2 - head * Math.cos(angle - Math.PI / 6), op.y2 - head * Math.sin(angle - Math.PI / 6))
      ctx.moveTo(op.x2, op.y2)
      ctx.lineTo(op.x2 - head * Math.cos(angle + Math.PI / 6), op.y2 - head * Math.sin(angle + Math.PI / 6))
      ctx.stroke()
    }
  }
  ctx.restore()
}

function render(): void {
  const canvas = canvasRef.value
  if (!canvas || !baseCanvas) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.drawImage(baseCanvas, 0, 0, canvas.width, canvas.height)

  ctx.save()
  ctx.scale(scale, scale)
  for (const op of ops.value) drawOp(ctx, op)
  if (draft.value) drawOp(ctx, draft.value)
  ctx.restore()

  if (cropStart && cropCurrent) {
    const x1 = Math.min(cropStart[0], cropCurrent[0]) * scale
    const y1 = Math.min(cropStart[1], cropCurrent[1]) * scale
    const x2 = Math.max(cropStart[0], cropCurrent[0]) * scale
    const y2 = Math.max(cropStart[1], cropCurrent[1]) * scale
    ctx.fillStyle = 'rgba(2, 6, 23, 0.62)'
    ctx.fillRect(0, 0, canvas.width, y1)
    ctx.fillRect(0, y2, canvas.width, canvas.height - y2)
    ctx.fillRect(0, y1, x1, y2 - y1)
    ctx.fillRect(x2, y1, canvas.width - x2, y2 - y1)
    ctx.strokeStyle = '#38bdf8'
    ctx.lineWidth = 1.5
    ctx.setLineDash([6, 4])
    ctx.strokeRect(x1, y1, x2 - x1, y2 - y1)
    ctx.setLineDash([])
  }
}

function toPoint(event: MouseEvent): Point {
  const rect = canvasRef.value!.getBoundingClientRect()
  return [(event.clientX - rect.left) / scale, (event.clientY - rect.top) / scale]
}

function applyCrop(): void {
  if (!cropStart || !cropCurrent) return
  const x = Math.round(Math.min(cropStart[0], cropCurrent[0]))
  const y = Math.round(Math.min(cropStart[1], cropCurrent[1]))
  const w = Math.round(Math.abs(cropCurrent[0] - cropStart[0]))
  const h = Math.round(Math.abs(cropCurrent[1] - cropStart[1]))
  cropStart = null
  cropCurrent = null
  if (w < 20 || h < 20) {
    render()
    return
  }
  const next = document.createElement('canvas')
  next.width = w
  next.height = h
  next.getContext('2d')!.drawImage(baseCanvas, x, y, w, h, 0, 0, w, h)
  baseCanvas = next
  imageW = w
  imageH = h
  ops.value = []
  draft.value = null
  strokeWidth.value = Math.max(2, Math.round(w / 350))
  layout()
  showToast('已裁剪')
}

function onMouseDown(event: MouseEvent): void {
  if (!baseCanvas) return
  const point = toPoint(event)
  if (tool.value === 'crop') {
    cropStart = point
    cropCurrent = point
    render()
    return
  }
  if (tool.value === 'text') {
    inlinePos.value = point
    inlineText.value = ''
    void nextTick(() => inlineInput.value?.focus())
    return
  }
  drawing = true
  if (tool.value === 'pen') {
    draft.value = { kind: 'pen', points: [point], color: color.value, width: strokeWidth.value }
  } else {
    draft.value = {
      kind: tool.value,
      x1: point[0],
      y1: point[1],
      x2: point[0],
      y2: point[1],
      color: color.value,
      width: strokeWidth.value
    }
  }
  render()
}

function onMouseMove(event: MouseEvent): void {
  if (cropStart) {
    cropCurrent = toPoint(event)
    render()
    return
  }
  if (!drawing || !draft.value) return
  const point = toPoint(event)
  const current = draft.value
  if (current.kind === 'pen') {
    const last = current.points[current.points.length - 1]
    if (Math.hypot(point[0] - last[0], point[1] - last[1]) > 1.5) current.points.push(point)
  } else if (current.kind !== 'text') {
    current.x2 = point[0]
    current.y2 = point[1]
  }
  render()
}

function onMouseUp(): void {
  if (cropStart) {
    applyCrop()
    return
  }
  if (!drawing) return
  drawing = false
  if (draft.value) ops.value.push(draft.value)
  draft.value = null
  render()
}

function undo(): void {
  ops.value.pop()
  render()
}

function reset(): void {
  if (!originalCanvas) return
  baseCanvas = cloneCanvas(originalCanvas)
  imageW = originalCanvas.width
  imageH = originalCanvas.height
  ops.value = []
  draft.value = null
  cropStart = null
  cropCurrent = null
  layout()
  showToast('已还原')
}

const inlineStyle = computed(() => {
  if (!inlinePos.value) return {}
  const canvas = canvasRef.value
  return {
    left: `${(canvas?.offsetLeft ?? 0) + inlinePos.value[0] * scale}px`,
    top: `${(canvas?.offsetTop ?? 0) + inlinePos.value[1] * scale}px`
  }
})

function commitInline(): void {
  if (!inlinePos.value) return
  const text = inlineText.value.trim()
  if (text) {
    ops.value.push({
      kind: 'text',
      x: inlinePos.value[0],
      y: inlinePos.value[1],
      text,
      color: color.value,
      size: Math.max(18, Math.round(imageW / 48))
    })
  }
  inlinePos.value = null
  inlineText.value = ''
  render()
}

function cancelInline(): void {
  inlineText.value = ''
  inlinePos.value = null
}

function wrapText(ctx: CanvasRenderingContext2D, text: string, maxWidth: number): string[] {
  const lines: string[] = []
  let line = ''
  for (const char of text) {
    if (char === '\n') {
      lines.push(line)
      line = ''
      continue
    }
    const test = line + char
    if (ctx.measureText(test).width > maxWidth && line) {
      lines.push(line)
      line = char
    } else {
      line = test
    }
  }
  if (line) lines.push(line)
  return lines
}

async function finish(saveToFile: boolean): Promise<void> {
  if (!baseCanvas) return
  const padding = Math.max(16, Math.round(imageW / 48))
  const fontSize = Math.max(18, Math.round(imageW / 72))
  const lineHeight = Math.round(fontSize * 1.5)
  const note = noteText.value.trim()
  const measure = document.createElement('canvas').getContext('2d')!
  measure.font = `500 ${fontSize}px "PingFang SC", "Microsoft YaHei", sans-serif`
  const lines = note ? wrapText(measure, note, imageW - padding * 2) : []
  const noteHeight = lines.length ? padding * 2 + lines.length * lineHeight : 0

  const out = document.createElement('canvas')
  out.width = imageW
  out.height = imageH + noteHeight
  const ctx = out.getContext('2d')!
  ctx.fillStyle = '#f8fafc'
  ctx.fillRect(0, 0, out.width, out.height)
  ctx.drawImage(baseCanvas, 0, 0)
  for (const op of ops.value) drawOp(ctx, op)
  if (lines.length) {
    ctx.fillStyle = '#0f172a'
    ctx.font = `500 ${fontSize}px "PingFang SC", "Microsoft YaHei", sans-serif`
    ctx.textBaseline = 'top'
    lines.forEach((line, index) => ctx.fillText(line, padding, imageH + padding + index * lineHeight))
  }

  await window.electronAPI.completeScreenshot({ dataUrl: out.toDataURL('image/png'), text: note, saveToFile })
  showToast(saveToFile ? '已保存到文件' : '已复制到剪贴板')
  window.setTimeout(() => window.electronAPI.closeScreenshot(), 500)
}

function close(): void {
  window.electronAPI.closeScreenshot()
}

function onKeyDown(event: KeyboardEvent): void {
  if (inlinePos.value) return
  if (event.key === 'Escape') {
    event.preventDefault()
    close()
  } else if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'z') {
    event.preventDefault()
    undo()
  }
}

onMounted(async () => {
  window.addEventListener('keydown', onKeyDown)
  window.addEventListener('resize', layout)
  try {
    const data = await window.electronAPI.getScreenshotInit()
    if (!data) {
      error.value = '没有可编辑的截图，请按 F1 重新截屏'
      return
    }
    const image = new Image()
    image.src = data.dataUrl
    await image.decode()
    imageW = image.naturalWidth
    imageH = image.naturalHeight
    baseCanvas = document.createElement('canvas')
    baseCanvas.width = imageW
    baseCanvas.height = imageH
    baseCanvas.getContext('2d')!.drawImage(image, 0, 0)
    originalCanvas = cloneCanvas(baseCanvas)
    strokeWidth.value = Math.max(2, Math.round(imageW / 350))
    await nextTick()
    layout()
  } catch (reason) {
    error.value = '截图加载失败'
    console.error(reason)
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeyDown)
  window.removeEventListener('resize', layout)
  if (toastTimer) window.clearTimeout(toastTimer)
})
</script>

<template>
  <div class="shot-app">
    <div v-if="error" class="error-view">
      <p>{{ error }}</p>
      <button class="btn btn-primary" @click="close">关闭</button>
    </div>
    <template v-else>
      <header class="shot-toolbar">
        <div class="group">
          <button
            v-for="item in tools"
            :key="item.key"
            class="tool"
            :class="{ active: tool === item.key }"
            :title="item.label"
            @click="tool = item.key"
          >
            <AppIcon :name="item.icon" />
          </button>
        </div>
        <div class="sep" />
        <div class="group colors">
          <button
            v-for="item in colors"
            :key="item"
            class="swatch"
            :class="{ active: color === item }"
            :style="{ background: item }"
            @click="color = item"
          />
        </div>
        <div class="sep" />
        <div class="group">
          <button class="tool" title="撤销 (Ctrl+Z)" @click="undo"><AppIcon name="undo" /></button>
          <button class="tool" title="还原截图" @click="reset"><AppIcon name="trash" /></button>
        </div>
        <div class="spacer" />
        <div class="group">
          <button class="btn btn-primary" @click="finish(false)"><AppIcon name="copy" /> 复制</button>
          <button class="btn" @click="finish(true)"><AppIcon name="save" /> 保存</button>
          <button class="btn-icon" title="关闭 (Esc)" @click="close"><AppIcon name="close" /></button>
        </div>
      </header>

      <div class="shot-body">
        <div ref="wrapRef" class="canvas-wrap">
          <canvas
            ref="canvasRef"
            @mousedown.prevent="onMouseDown"
            @mousemove="onMouseMove"
            @mouseup="onMouseUp"
            @mouseleave="onMouseUp"
          />
          <div v-if="inlinePos" class="inline-editor" :style="inlineStyle">
            <input
              ref="inlineInput"
              v-model="inlineText"
              placeholder="输入文字后回车"
              @keydown.enter.prevent.stop="commitInline"
              @keydown.esc.prevent.stop="cancelInline"
              @blur="commitInline"
            />
          </div>
          <div v-if="tool === 'crop'" class="crop-hint">拖动鼠标选择区域，松开完成裁剪</div>
        </div>

        <div class="note-area">
          <div class="note-label"><AppIcon name="text" /> 输入说明文字（将拼接进图片）</div>
          <textarea
            v-model="noteText"
            spellcheck="false"
            placeholder="例如：此处按钮点击无效、需要优化的样式、操作步骤说明…"
          />
        </div>
      </div>

      <div v-if="toast" class="toast">{{ toast }}</div>
    </template>
  </div>
</template>

<style scoped>
.shot-app {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #070b16;
  overflow: hidden;
}

.shot-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 54px;
  padding: 0 14px;
  border-bottom: 1px solid var(--border);
  background: rgba(13, 20, 40, 0.96);
  user-select: none;
}

.group {
  display: flex;
  align-items: center;
  gap: 3px;
}

.spacer {
  flex: 1;
}

.sep {
  width: 1px;
  height: 22px;
  background: var(--border);
}

.tool {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  border: 1px solid transparent;
  border-radius: 8px;
  background: transparent;
  color: var(--muted);
}

.tool:hover {
  background: rgba(129, 168, 230, 0.12);
  color: var(--text);
}

.tool.active {
  border-color: rgba(56, 189, 248, 0.5);
  background: rgba(56, 189, 248, 0.16);
  color: #7dd3fc;
}

.colors .swatch {
  width: 18px;
  height: 18px;
  padding: 0;
  border: 2px solid transparent;
  border-radius: 50%;
  box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.18);
}

.colors .swatch.active {
  border-color: #0b1020;
  box-shadow: 0 0 0 2px #7dd3fc;
}

.shot-body {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-height: 0;
}

.canvas-wrap {
  position: relative;
  display: flex;
  flex: 1;
  align-items: center;
  justify-content: center;
  min-height: 0;
  padding: 24px;
  overflow: hidden;
  background:
    radial-gradient(circle at 50% 40%, rgba(56, 189, 248, 0.07), transparent 55%),
    #070b16;
}

.canvas-wrap canvas {
  display: block;
  cursor: crosshair;
  border: 1px solid rgba(129, 168, 230, 0.18);
  border-radius: 3px;
  box-shadow: 0 18px 60px rgba(0, 0, 0, 0.55);
  max-width: 100%;
  max-height: 100%;
}

.inline-editor {
  position: absolute;
  z-index: 5;
}

.inline-editor input {
  width: 190px;
  height: 30px;
  padding: 0 8px;
  border: 1px solid #38bdf8;
  border-radius: 5px;
  background: rgba(7, 11, 22, 0.95);
  color: #fff;
  font-size: 13px;
  outline: none;
}

.crop-hint {
  position: absolute;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  padding: 6px 14px;
  border: 1px solid rgba(56, 189, 248, 0.35);
  border-radius: 999px;
  background: rgba(7, 11, 22, 0.85);
  color: #7dd3fc;
  font-size: 12px;
  pointer-events: none;
}

.note-area {
  height: 132px;
  flex: none;
  padding: 10px 18px 12px;
  background: linear-gradient(180deg, #f8fafc, #eef2f7);
  color: #0f172a;
}

.note-label {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
  color: #64748b;
  font-size: 11px;
}

.note-label :deep(.app-icon) {
  width: 14px;
  height: 14px;
}

.note-area textarea {
  width: 100%;
  height: 82px;
  padding: 0;
  border: none;
  outline: none;
  resize: none;
  background: transparent;
  color: #0f172a;
  font-size: 14px;
  line-height: 1.6;
  user-select: text;
}

.error-view {
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: var(--muted);
}
</style>
