import * as cp from 'child_process'
import * as fs from 'fs/promises'
import * as os from 'os'
import * as path from 'path'
import * as vscode from 'vscode'

type ContextShrinkConfig = {
  binaryPath: string
  level: number
  maxTokens: number
  outputFile: string
}

type GeneratedContext = {
  approxTokens: number
  outputFile: string
}

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand('contextshrink.generateContext', async () => {
      const generated = await generateContext(context)
      await openContextFile(generated.outputFile)
      await vscode.env.clipboard.writeText(buildContextPrompt(generated.outputFile))
      showSuccessMessage(generated, 'Prompt copied. Paste it into Copilot Chat, ChatGPT, or Codex in VS Code.')
    }),
    vscode.commands.registerCommand('contextshrink.generateAndAsk', async () => {
      const generated = await generateContext(context)
      const xml = await fs.readFile(generated.outputFile, 'utf8')
      const prompt = buildContextPrompt(generated.outputFile, xml)
      await openContextFile(generated.outputFile)
      await vscode.env.clipboard.writeText(prompt)
      await openChat(prompt)
      showSuccessMessage(generated, 'Chat opened when available. Prompt also copied.')
    }),
    vscode.commands.registerCommand('contextshrink.copyContext', async () => {
      const generated = await generateContext(context)
      const xml = await fs.readFile(generated.outputFile, 'utf8')
      await vscode.env.clipboard.writeText(buildContextPrompt(generated.outputFile, xml))
      showSuccessMessage(generated, 'Full XML prompt copied. Paste it into Copilot Chat, ChatGPT, or Codex in VS Code.')
    }),
    vscode.commands.registerCommand('contextshrink.openContext', async () => {
      const outputFile = getConfig().outputFile
      await openContextFile(outputFile)
    })
  )
}

export function deactivate() {}

async function generateContext(context: vscode.ExtensionContext): Promise<GeneratedContext> {
  const workspaceRoot = getWorkspaceRoot()
  const config = getConfig()
  const binaryPath = await resolveBinaryPath(context, config.binaryPath)

  await runContextShrink(binaryPath, [
    workspaceRoot,
    '--max-tokens',
    String(config.maxTokens),
    '--level',
    String(config.level),
    '--output',
    'file',
    '--output-file',
    config.outputFile
  ])

  const xml = await fs.readFile(config.outputFile, 'utf8')
  return {
    approxTokens: approximateTokenCount(xml),
    outputFile: config.outputFile
  }
}

function getWorkspaceRoot(): string {
  const folder = vscode.workspace.workspaceFolders?.[0]
  if (!folder) {
    throw new Error('Open a workspace folder before running ContextShrink.')
  }
  return folder.uri.fsPath
}

function getConfig(): ContextShrinkConfig {
  const config = vscode.workspace.getConfiguration('contextshrink')
  return {
    binaryPath: config.get<string>('binaryPath', ''),
    level: config.get<number>('level', 2),
    maxTokens: config.get<number>('maxTokens', 12000),
    outputFile: expandHome(config.get<string>('outputFile', '/tmp/contextshrink-vscode.xml'))
  }
}

async function resolveBinaryPath(context: vscode.ExtensionContext, configuredPath: string): Promise<string> {
  const expandedConfiguredPath = expandHome(configuredPath.trim())
  if (expandedConfiguredPath) {
    return expandedConfiguredPath
  }

  const envBinary = expandHome(process.env.CONTEXTSHRINK_BIN?.trim() ?? '')
  if (envBinary) {
    return envBinary
  }

  const pathBinary = await findExecutableOnPath('contextshrink')
  if (pathBinary) {
    return pathBinary
  }

  const candidates = [
    path.resolve(context.extensionPath, '..', '..', 'target', 'release', 'contextshrink'),
    path.join(os.homedir(), 'dev', 'context-shrink', 'target', 'release', 'contextshrink')
  ]

  for (const candidate of candidates) {
    if (await isExecutable(candidate)) {
      return candidate
    }
  }

  return 'contextshrink'
}

async function isExecutable(filePath: string): Promise<boolean> {
  try {
    await fs.access(filePath, fs.constants.X_OK)
    return true
  } catch {
    return false
  }
}

async function findExecutableOnPath(name: string): Promise<string | undefined> {
  const pathValue = process.env.PATH ?? ''
  for (const directory of pathValue.split(path.delimiter)) {
    if (!directory) {
      continue
    }

    const candidate = path.join(directory, name)
    if (await isExecutable(candidate)) {
      return candidate
    }
  }

  return undefined
}

function expandHome(value: string): string {
  if (value === '~') {
    return os.homedir()
  }
  if (value.startsWith('~/')) {
    return path.join(os.homedir(), value.slice(2))
  }
  return value
}

function runContextShrink(binaryPath: string, args: string[]): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = cp.spawn(binaryPath, args, {
      cwd: getWorkspaceRoot(),
      env: process.env
    })

    let stderr = ''

    child.stderr.on('data', chunk => {
      stderr += chunk.toString()
    })

    child.on('error', error => {
      reject(new Error(`Could not run ContextShrink: ${error.message}`))
    })

    child.on('close', code => {
      if (code === 0) {
        resolve()
        return
      }
      reject(new Error(`ContextShrink failed with exit code ${code}: ${stderr}`))
    })
  })
}

async function openContextFile(outputFile: string): Promise<void> {
  const document = await vscode.workspace.openTextDocument(vscode.Uri.file(outputFile))
  await vscode.window.showTextDocument(document, { preview: false })
}

async function openChat(prompt: string): Promise<void> {
  try {
    await vscode.commands.executeCommand('workbench.action.chat.open', prompt)
  } catch {
    await vscode.commands.executeCommand('workbench.action.chat.open')
  }
}

function buildContextPrompt(outputFile: string, xml?: string): string {
  if (xml) {
    return `Use this ContextShrink XML as compressed repository context for Copilot Chat, ChatGPT, or Codex in VS Code, then answer my next question.\n\n${xml}`
  }

  return `Use the ContextShrink XML opened at ${outputFile} as compressed repository context for Copilot Chat, ChatGPT, or Codex in VS Code, then answer my next question.`
}

function approximateTokenCount(text: string): number {
  return Math.ceil(text.length / 4)
}

function showSuccessMessage(generated: GeneratedContext, nextStep: string): void {
  vscode.window.showInformationMessage(
    `ContextShrink wrote ${generated.outputFile} (~${generated.approxTokens} tokens). ${nextStep}`
  )
}
