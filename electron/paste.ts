import { exec } from 'node:child_process'
import { platform } from 'node:process'

const WINDOWS_SCRIPT = `
Add-Type -AssemblyName System.Windows.Forms | Out-Null
[System.Windows.Forms.SendKeys]::SendWait('^v')
Write-Output 'PASTED'
`

const MAC_SCRIPT = `
tell application "System Events"
  keystroke "v" using command down
end tell
return "PASTED"
`

function run(command: string, timeout: number): Promise<string> {
  return new Promise((resolve) => {
    exec(command, { timeout, windowsHide: true }, (error, stdout) => {
      if (error) console.warn('模拟粘贴执行失败:', error.message)
      resolve(stdout ?? '')
    })
  })
}

export async function pasteToFocusedInput(): Promise<boolean> {
  try {
    switch (platform) {
      case 'win32': {
        const encoded = Buffer.from(WINDOWS_SCRIPT, 'utf16le').toString('base64')
        const output = await run(
          `powershell -NoProfile -NonInteractive -EncodedCommand ${encoded}`,
          3000
        )
        return output.includes('PASTED')
      }
      case 'darwin': {
        const output = await run(`osascript -e ${JSON.stringify(MAC_SCRIPT)}`, 3000)
        return output.includes('PASTED')
      }
      default: {
        await run('xdotool key ctrl+v', 1500)
        return true
      }
    }
  } catch (error) {
    console.error('粘贴到聚焦输入框失败', error)
    return false
  }
}
