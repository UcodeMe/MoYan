import process from 'node:process'

const major = Number(process.versions.node.split('.')[0])

if (major < 18) {
  console.error('')
  console.error(`[MoYan] 当前 Node.js 版本为 ${process.versions.node}，本项目要求 Node.js >= 18。`)
  console.error('[MoYan] 检测到你的系统使用 nvm 管理多版本，请先执行：')
  console.error('[MoYan]     nvm use 22.21.1')
  console.error('[MoYan] 然后重新运行命令。')
  console.error('')
  process.exit(1)
}
