import * as path from 'path'

import { ESLint } from 'eslint'
import * as glob from 'glob'

import {
  validateEnvPrint as validateEnvPrintBinding,
  validateConfigPrint as validateConfigPrintBinding,
  validatePackagePrint as validatePackagePrintBinding,
  validateRecommendPrint as validateRecommendPrintBinding,
  validateEnv as validateEnvBinding,
  validateConfig as validateConfigBinding,
  validatePackage as validatePackageBinding,
  validateRecommend as validateRecommendBinding,
  MessageKind,
  Message,
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
      validateEnvPrint()
      await validateConfigPrint(ctx.initialConfig, ctx.helper)
      validatePackagePrint(appPath, nodeModulesPath)
      validateRecommendPrint(appPath)
      await validateEslintPrint(ctx.initialConfig, chalk)
    },
  })
}

async function getValidateConfigParams(projectConfig: any, helper: any) {
  const configStr = JSON.stringify(projectConfig, (_, v) => {
    if (typeof v === 'function') {
      return '__function__'
    }
    return v
  })
  let remoteSchemaUrl = 'https://raw.githubusercontent.com/NervJS/taro-doctor/main/assets/config_schema.json'
  let useRemoteSchema = true
  const homedir = helper.getUserHomeDir()
  if (homedir) {
    const taroConfigPath = path.join(homedir, helper.TARO_CONFIG_FOLDER)
    const taroConfig = path.join(taroConfigPath, helper.TARO_BASE_CONFIG)
    if (helper.fs.existsSync(taroConfig)) {
      const config = await helper.fs.readJSON(taroConfig)
      remoteSchemaUrl = config && config.remoteConfigSchemaUrl ? config.remoteConfigSchemaUrl : remoteSchemaUrl
      useRemoteSchema = config && config.useRemoteConfigSchema ? config.useRemoteConfigSchema : useRemoteSchema
    } else {
      await helper.fs.createFile(taroConfig)
      await helper.fs.writeJSON(taroConfig, { remoteSchemaUrl, useRemoteSchema })
    }
  }
  return { configStr, remoteSchemaUrl, useRemoteSchema }
}

export async function validateConfig(projectConfig: any, helper: any): Promise<ValidateResult> {
  const { configStr, remoteSchemaUrl, useRemoteSchema } = await getValidateConfigParams(projectConfig, helper)
  return validateConfigBinding(configStr, remoteSchemaUrl, useRemoteSchema)
}

export async function validateConfigPrint(projectConfig: any, helper: any): Promise<boolean> {
  const { configStr, remoteSchemaUrl, useRemoteSchema } = await getValidateConfigParams(projectConfig, helper)
  return validateConfigPrintBinding(configStr, remoteSchemaUrl, useRemoteSchema)
}

export function validateEnv(): ValidateResult {
  return validateEnvBinding()
}

export function validateEnvPrint(): boolean {
  return validateEnvPrintBinding()
}

export function validatePackage(appPath: string, nodeModulesPath: string): ValidateResult {
  return validatePackageBinding(appPath, nodeModulesPath)
}

export function validatePackagePrint(appPath: string, nodeModulesPath: string): boolean {
  return validatePackagePrintBinding(appPath, nodeModulesPath)
}

export function validateRecommend(appPath: string): ValidateResult {
  return validateRecommendBinding(appPath)
}

export function validateRecommendPrint(appPath: string): boolean {
  return validateRecommendPrintBinding(appPath)
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

export {
  MessageKind,
  ValidateResult,
  Message
}
