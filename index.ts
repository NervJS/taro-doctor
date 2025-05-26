import * as path from 'path'

import { ESLint, loadESLint } from 'eslint'
import * as glob from 'glob'

import * as stylelint from 'stylelint'
import {
  Message,
  MessageKind,
  validateConfig as validateConfigBinding,
  validateConfigPrint as validateConfigPrintBinding,
  validateEnv as validateEnvBinding,
  validateEnvPrint as validateEnvPrintBinding,
  validatePackage as validatePackageBinding,
  validatePackagePrint as validatePackagePrintBinding,
  validateRecommend as validateRecommendBinding,
  validateRecommendPrint as validateRecommendPrintBinding,
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
      await validateStylelintPrint(ctx.initialConfig, chalk)
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

export async function validateStylelintPrint(projectConfig, chalk): Promise<boolean> {
  const result = await validateStylelintCore(projectConfig, chalk)
  let is_valid = result.isValid
  let report = result.messages[0].content
  console.log(`\u{1F3AF} 检查 Stylelint (以下为 Stylelint 的输出)！`)
  if (is_valid) {
    console.log(`${chalk.green('[\u{2713}]')} Stylelint 检查通过！`)
  } else {
    console.log(report)
  }
  return is_valid
}

async function validateStylelintCore(projectConfig, chalk): Promise<ValidateResult> {
  const appPath = process.cwd()
  const linterResult = await stylelint.lint({
    files: path.join(appPath, projectConfig.sourceRoot, '**/*.{css,less,scss,sass}'),
    configBasedir: appPath,
    formatter: 'string'
  })
  let report = linterResult.report
  let is_valid = true
  for (const result of linterResult.results) {
    if (result.warnings.length > 0) {
      is_valid = false
      break
    }
  }
  if (is_valid) {
    report = `${chalk.green('[\u{2713}]')} Stylelint 检查通过！`
  }
  return {
    isValid: is_valid,
    messages: [
      {
        kind: is_valid ? MessageKind.Success : MessageKind.Error,
        content: report,
      },
    ],
  }
}

async function validateEslintCore(projectConfig, chalk): Promise<ValidateResult> {
  const appPath = process.cwd()
  const legacyConfigPattern = glob.sync(path.join(appPath, '.eslintrc*'))
  const flatConfigPattern = glob.sync(path.join(appPath, 'eslint.config.{js,cjs,mjs}'))
  const useFlatConfig = Boolean(flatConfigPattern.length)

  const cwd = process.cwd()

  const flatConfig: ESLint.Options = {
    cwd,
  }

  // 兼容 eslint8
  const legacyConfig: ESLint.LegacyOptions = {
    cwd,
    useEslintrc: Boolean(legacyConfigPattern.length),
    baseConfig: {
      extends: [`taro/${projectConfig.framework}`],
    },
  }

  const ESLint = await loadESLint({ useFlatConfig })
  const options = useFlatConfig ? flatConfig : legacyConfig
  const eslintCli = new ESLint(options as any)

  const sourceFiles = path.join(cwd, projectConfig.sourceRoot, '**/*.{js,ts,jsx,tsx}')
  const report = await eslintCli.lintFiles([sourceFiles])
  const formatter = await eslintCli.loadFormatter()
  let rawReport = await formatter.format(report)
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

export { Message, MessageKind, ValidateResult }
