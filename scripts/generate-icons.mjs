import fs from 'node:fs'
import path from 'node:path'
import zlib from 'node:zlib'

const OUT = path.join(process.cwd(), 'build', 'icons')

const crcTable = (() => {
  const table = new Uint32Array(256)
  for (let n = 0; n < 256; n++) {
    let c = n
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1
    table[n] = c >>> 0
  }
  return table
})()

function crc32(buf) {
  let c = 0xffffffff
  for (let i = 0; i < buf.length; i++) c = crcTable[(c ^ buf[i]) & 0xff] ^ (c >>> 8)
  return (c ^ 0xffffffff) >>> 0
}

function chunk(type, data) {
  const len = Buffer.alloc(4)
  len.writeUInt32BE(data.length, 0)
  const name = Buffer.from(type, 'ascii')
  const crc = Buffer.alloc(4)
  crc.writeUInt32BE(crc32(Buffer.concat([name, data])), 0)
  return Buffer.concat([len, name, data, crc])
}

function encodePng(size, rgba) {
  const stride = size * 4
  const raw = Buffer.alloc((stride + 1) * size)
  for (let y = 0; y < size; y++) {
    raw[y * (stride + 1)] = 0
    rgba.copy(raw, y * (stride + 1) + 1, y * stride, (y + 1) * stride)
  }
  const ihdr = Buffer.alloc(13)
  ihdr.writeUInt32BE(size, 0)
  ihdr.writeUInt32BE(size, 4)
  ihdr[8] = 8
  ihdr[9] = 6
  const png = Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk('IHDR', ihdr),
    chunk('IDAT', zlib.deflateSync(raw, { level: 9 })),
    chunk('IEND', Buffer.alloc(0))
  ])
  return png
}

const clamp01 = (v) => Math.max(0, Math.min(1, v))
const smoothstep = (e0, e1, x) => {
  const t = clamp01((x - e0) / (e1 - e0))
  return t * t * (3 - 2 * t)
}
const lerp = (a, b, t) => a + (b - a) * t

function sdRoundRect(px, py, cx, cy, hw, hh, r) {
  const qx = Math.abs(px - cx) - (hw - r)
  const qy = Math.abs(py - cy) - (hh - r)
  return Math.hypot(Math.max(qx, 0), Math.max(qy, 0)) + Math.min(Math.max(qx, qy), 0) - r
}

function sdSegment(px, py, ax, ay, bx, by) {
  const pax = px - ax
  const pay = py - ay
  const bax = bx - ax
  const bay = by - ay
  const len2 = bax * bax + bay * bay
  const h = len2 === 0 ? 0 : clamp01((pax * bax + pay * bay) / len2)
  return Math.hypot(pax - h * bax, pay - h * bay)
}

function render(size) {
  const rgba = Buffer.alloc(size * size * 4)
  const aa = 1.4 / size

  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const u = (x + 0.5) / size
      const v = (y + 0.5) / size

      let cr = 0
      let cg = 0
      let cb = 0
      let ca = 0
      const over = (r, g, b, a) => {
        if (a <= 0) return
        const out = a + ca * (1 - a)
        if (out <= 0) return
        cr = (r * a + cr * ca * (1 - a)) / out
        cg = (g * a + cg * ca * (1 - a)) / out
        cb = (b * a + cb * ca * (1 - a)) / out
        ca = out
      }

      const dBg = sdRoundRect(u, v, 0.5, 0.5, 0.5, 0.5, 0.22)
      const aBg = 1 - smoothstep(-aa, aa, dBg)
      const g = clamp01(u * 0.4 + v * 0.6)
      const glow = Math.max(0, 1 - Math.hypot(u - 0.5, v - 0.44) / 0.55) * 0.12
      over(
        lerp(10, 20, g) + 18 * glow,
        lerp(16, 32, g) + 90 * glow,
        lerp(34, 64, g) + 130 * glow,
        aBg
      )

      const dBorder = Math.abs(sdRoundRect(u, v, 0.5, 0.5, 0.435, 0.435, 0.185))
      over(56, 189, 248, (1 - smoothstep(0.0015, 0.0045, dBorder)) * 0.34)

      const dM = Math.min(
        sdSegment(u, v, 0.27, 0.70, 0.27, 0.30),
        sdSegment(u, v, 0.27, 0.30, 0.50, 0.56),
        sdSegment(u, v, 0.50, 0.56, 0.73, 0.30),
        sdSegment(u, v, 0.73, 0.30, 0.73, 0.70)
      )
      over(34, 211, 238, (1 - smoothstep(0.045, 0.115, dM)) * 0.3)
      const t = clamp01((u - 0.27) / 0.46)
      over(
        lerp(34, 129, t),
        lerp(211, 140, t),
        lerp(238, 248, t),
        1 - smoothstep(0.036, 0.052, dM)
      )

      const d1 = Math.hypot(u - 0.78, v - 0.24)
      over(103, 232, 249, 1 - smoothstep(0.012, 0.020, d1))
      const d2 = Math.hypot(u - 0.22, v - 0.76)
      over(129, 140, 248, (1 - smoothstep(0.012, 0.020, d2)) * 0.85)

      const i = (y * size + x) * 4
      rgba[i] = Math.round(clamp01(cr / 255) * 255)
      rgba[i + 1] = Math.round(clamp01(cg / 255) * 255)
      rgba[i + 2] = Math.round(clamp01(cb / 255) * 255)
      rgba[i + 3] = Math.round(clamp01(ca) * 255)
    }
  }
  return encodePng(size, rgba)
}

function encodeIco(entries) {
  const count = entries.length
  const header = Buffer.alloc(6)
  header.writeUInt16LE(0, 0)
  header.writeUInt16LE(1, 2)
  header.writeUInt16LE(count, 4)
  const dir = Buffer.alloc(16 * count)
  let offset = 6 + 16 * count
  entries.forEach((entry, i) => {
    const base = i * 16
    dir[base] = entry.size >= 256 ? 0 : entry.size
    dir[base + 1] = entry.size >= 256 ? 0 : entry.size
    dir[base + 2] = 0
    dir[base + 3] = 0
    dir.writeUInt16LE(1, base + 4)
    dir.writeUInt16LE(32, base + 6)
    dir.writeUInt32LE(entry.png.length, base + 8)
    dir.writeUInt32LE(offset, base + 12)
    offset += entry.png.length
  })
  return Buffer.concat([header, dir, ...entries.map((entry) => entry.png)])
}

function encodeIcns(entries) {
  const body = Buffer.concat(
    entries.map((entry) => {
      const head = Buffer.alloc(8)
      head.write(entry.type, 0, 'ascii')
      head.writeUInt32BE(entry.png.length + 8, 4)
      return Buffer.concat([head, entry.png])
    })
  )
  const header = Buffer.alloc(8)
  header.write('icns', 0, 'ascii')
  header.writeUInt32BE(body.length + 8, 4)
  return Buffer.concat([header, body])
}

fs.mkdirSync(OUT, { recursive: true })
for (const size of [1024, 512, 256, 128, 64, 32, 16]) {
  const name = size === 1024 ? 'icon.png' : `icon-${size}.png`
  fs.writeFileSync(path.join(OUT, name), render(size))
}
const icoSizes = [16, 32, 48, 64, 128, 256]
fs.writeFileSync(
  path.join(OUT, 'icon.ico'),
  encodeIco(icoSizes.map((size) => ({ size, png: render(size) })))
)
fs.writeFileSync(
  path.join(OUT, 'icon.icns'),
  encodeIcns([
    { type: 'ic07', png: render(128) },
    { type: 'ic08', png: render(256) },
    { type: 'ic09', png: render(512) },
    { type: 'ic10', png: render(1024) }
  ])
)
fs.copyFileSync(path.join(OUT, 'icon-32.png'), path.join(OUT, 'tray.png'))
console.log('图标已生成:', OUT)
