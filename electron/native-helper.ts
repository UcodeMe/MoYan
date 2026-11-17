import { spawn, execFile, type ChildProcess } from 'node:child_process'
import fs from 'node:fs'
import path from 'node:path'
import { app } from 'electron'
import { platform } from 'node:process'

const HELPER_VERSION = 'v1'

// C# 5 语法（Windows 自带 csc.exe 可直接编译，无第三方依赖）
const HELPER_SOURCE = `
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Threading;

static class Program
{
    [DllImport("user32.dll")]
    static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")]
    static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint processId);
    [DllImport("user32.dll")]
    static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")]
    static extern bool IsIconic(IntPtr hWnd);
    [DllImport("user32.dll")]
    static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")]
    static extern void SwitchToThisWindow(IntPtr hWnd, bool fAltTab);
    [DllImport("user32.dll")]
    static extern void keybd_event(byte bVk, byte bScan, uint dwFlags, UIntPtr dwExtraInfo);

    const uint KEYEVENTF_KEYUP = 2;
    const int SW_RESTORE = 9;

    static string ownerExe;

    static void Main(string[] args)
    {
        if (args.Length >= 2 && args[0] == "paste")
        {
            Paste(args[1]);
            return;
        }
        if (args.Length >= 2 && args[0] == "watch")
        {
            Watch(uint.Parse(args[1]));
        }
    }

    static void Watch(uint ownerPid)
    {
        try
        {
            using (Process owner = Process.GetProcessById((int)ownerPid))
            {
                ownerExe = owner.MainModule.FileName;
            }
        }
        catch
        {
            ownerExe = null;
        }
        IntPtr last = IntPtr.Zero;
        int ticks = 0;
        while (true)
        {
            ticks++;
            if (ticks % 20 == 0 && !OwnerAlive(ownerPid)) return;
            IntPtr hwnd = GetForegroundWindow();
            if (hwnd != IntPtr.Zero && hwnd != last && IsWindowVisible(hwnd))
            {
                uint pid;
                GetWindowThreadProcessId(hwnd, out pid);
                if (pid != ownerPid && NotOwnedByApp(pid))
                {
                    last = hwnd;
                    Console.WriteLine(hwnd.ToInt64());
                    Console.Out.Flush();
                }
            }
            Thread.Sleep(150);
        }
    }

    static bool OwnerAlive(uint pid)
    {
        try { Process.GetProcessById((int)pid); return true; }
        catch { return false; }
    }

    static bool NotOwnedByApp(uint pid)
    {
        if (ownerExe == null) return true;
        try
        {
            using (Process p = Process.GetProcessById((int)pid))
            {
                string exe = p.MainModule.FileName;
                return !string.Equals(exe, ownerExe, StringComparison.OrdinalIgnoreCase);
            }
        }
        catch { return true; }
    }

    static void Paste(string hwndText)
    {
        IntPtr target = IntPtr.Zero;
        long parsed;
        if (long.TryParse(hwndText, out parsed)) target = new IntPtr(parsed);
        if (target != IntPtr.Zero && IsWindowVisible(target))
        {
            if (IsIconic(target)) ShowWindow(target, SW_RESTORE);
            SwitchToThisWindow(target, false);
            Thread.Sleep(80);
        }
        keybd_event(0x11, 0, 0, UIntPtr.Zero);
        keybd_event(0x56, 0, 0, UIntPtr.Zero);
        Thread.Sleep(20);
        keybd_event(0x56, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
        keybd_event(0x11, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
        Console.WriteLine("PASTED");
    }
}
`

const sourceFile = (): string => path.join(app.getPath('userData'), `moyan-helper-${HELPER_VERSION}.cs`)
const exeFile = (): string => path.join(app.getPath('userData'), `moyan-helper-${HELPER_VERSION}.exe`)

let watcher: ChildProcess | null = null
let lastHwnd = ''
let helperReady = false
let shouldRun = false
let restartCount = 0

function findCsc(): string | null {
  const candidates = [
    'C:/Windows/Microsoft.NET/Framework64/v4.0.30319/csc.exe',
    'C:/Windows/Microsoft.NET/Framework/v4.0.30319/csc.exe'
  ]
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) return candidate
  }
  return null
}

function compileHelper(): Promise<boolean> {
  const exe = exeFile()
  if (fs.existsSync(exe)) {
    helperReady = true
    return Promise.resolve(true)
  }
  const csc = findCsc()
  if (!csc) {
    console.warn('未找到 csc.exe，双击粘贴将使用降级方案')
    return Promise.resolve(false)
  }
  fs.writeFileSync(sourceFile(), HELPER_SOURCE, 'utf-8')
  return new Promise((resolve) => {
    execFile(
      csc,
      ['/nologo', '/target:exe', `/out:${exe}`, sourceFile()],
      { timeout: 20000, windowsHide: true },
      (error) => {
        if (error) {
          console.warn('编译前台窗口助手失败:', error.message)
          helperReady = false
          resolve(false)
          return
        }
        helperReady = fs.existsSync(exe)
        resolve(helperReady)
      }
    )
  })
}

export async function startForegroundWatcher(): Promise<void> {
  if (platform !== 'win32') return
  shouldRun = true
  const ok = await compileHelper()
  if (!ok) return
  stopForegroundWatcher()
  try {
    watcher = spawn(exeFile(), ['watch', String(process.pid)], {
      windowsHide: true,
      stdio: ['ignore', 'pipe', 'ignore']
    })
  } catch (error) {
    console.warn('前台窗口助手启动失败', error)
    return
  }
  let buffer = ''
  watcher.stdout?.setEncoding('utf-8')
  watcher.stdout?.on('data', (chunk: string) => {
    buffer += chunk
    const lines = buffer.split(/\r?\n/)
    buffer = lines.pop() ?? ''
    for (const line of lines) {
      const value = line.trim()
      if (/^\d+$/.test(value)) lastHwnd = value
    }
  })
  watcher.on('exit', () => {
    watcher = null
    if (shouldRun && restartCount < 5) {
      restartCount++
      void startForegroundWatcher()
    }
  })
}

export function stopForegroundWatcher(): void {
  shouldRun = false
  if (watcher) {
    watcher.removeAllListeners('exit')
    watcher.kill()
    watcher = null
  }
}

export function canPasteToRecordedWindow(): boolean {
  return platform === 'win32' && helperReady && lastHwnd !== '' && !!watcher
}

export function pasteToRecordedWindow(): Promise<boolean> {
  return new Promise((resolve) => {
    if (!helperReady || lastHwnd === '') {
      resolve(false)
      return
    }
    try {
      const child = spawn(exeFile(), ['paste', lastHwnd], {
        windowsHide: true,
        stdio: 'ignore'
      })
      child.on('exit', () => resolve(true))
      child.on('error', (error) => {
        console.warn('粘贴助手执行失败', error)
        resolve(false)
      })
    } catch (error) {
      console.warn('粘贴助手启动失败', error)
      resolve(false)
    }
  })
}
