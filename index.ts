import * as path from 'path'

import { ESLint } from 'eslint'
import * as glob from 'glob'

import {
  validateEnvPrint,
  validateConfigPrint,
  validatePackagePrint,
  validateRecommendPrint,
  MessageKind,
  ValidateResult,
} from './js-binding'

export default (ctx) => {
  ctx.registerCommand({
    name: 'doctor',
    async fn() {
      const { appPath, nodeModulesPath, configPath } = ctx.paths
      const { fs, chalk, PROJECT_CONFIG } = ctx.helper

      if (!configPath || !fs.existsSync(configPath)) {
        console.log(chalk.red(`找不到项目配置文件${PROJECT_CONFIG}，请确定当前目录是 Taro 项目根目录!`))
        process.exit(1)
      }
      const configStr = JSON.stringify(ctx.initialConfig, (_, v) => {
        if (typeof v === 'function') {
          return '__function__'
        }
        return v
      })
      validateEnvPrint()
      validateConfigPrint(configStr)
      validatePackagePrint(appPath, nodeModulesPath)
      validateRecommendPrint(appPath)
      await validateEslintPrint(ctx.initialConfig, chalk)
    },
  })
}

export async function validateEslint(projectConfig, chalk): Promise<ValidateResult> {
  const result = await validateEslintCore(projectConfig, chalk)
  result.messages.unshift({
    kind: MessageKind.Info,
    content: `\u{1F3AF} 检查 ESLint (以下为 ESLint 的输出)！`,
  })
  return result
}

export async function validateEslintPrint(projectConfig, chalk): Promise<boolean> {
  const result = await validateEslintCore(projectConfig, chalk)
  let is_valid = result.isValid
  let rawReport = result.messages[0].content
  console.log(`\u{1F3AF} 检查 ESLint (以下为 ESLint 的输出)！`)
  if (is_valid) {
    console.log(`${chalk.green('[\u{2713}]')} Eslint 检查通过！`)
  } else {
    console.log(rawReport)
  }
  return is_valid
}

async function validateEslintCore(projectConfig, chalk): Promise<ValidateResult> {
  const appPath = process.cwd()
  const globPattern = glob.sync(path.join(appPath, '.eslintrc*'))

  const eslintCli = new ESLint({
    cwd: process.cwd(),
    useEslintrc: Boolean(globPattern.length),
    baseConfig: {
      extends: [`taro/${projectConfig.framework}`],
    },
  })

  const sourceFiles = path.join(process.cwd(), projectConfig.sourceRoot, '**/*.{js,ts,jsx,tsx}')
  const report = await eslintCli.lintFiles([sourceFiles])
  const formatter = await eslintCli.loadFormatter()
  let rawReport = formatter.format(report)
  let is_valid = true
  if (rawReport) {
    is_valid = false
  }
  if (is_valid) {
    rawReport = `${chalk.green('[\u{2713}]')} Eslint 检查通过！`
  }
  return {
    isValid: is_valid,
    messages: [
      {
        kind: is_valid ? MessageKind.Success : MessageKind.Error,
        content: rawReport,
      },
    ],
  }
}
