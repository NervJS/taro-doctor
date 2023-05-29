import * as path from 'path'

import { ESLint } from 'eslint'
import * as glob from 'glob'
import { IPluginContext } from '@tarojs/service'

import {
  validateEnv,
  validateConfig,
  validatePackage,
  validateRecommend
} from './js-binding'

export default (ctx: IPluginContext) => {
	ctx.registerCommand({
    name: 'dd',
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
      validateEnv()
      validateConfig(configStr)
      validatePackage(appPath, nodeModulesPath)
      validateRecommend(appPath)
      await validateEslint(ctx.initialConfig, chalk)
    },
  })
}

async function validateEslint(projectConfig, chalk) {
  const appPath = process.cwd()
  const globPattern = glob.sync(path.join(appPath, '.eslintrc*'))

  const eslintCli = new ESLint({
    cwd: process.cwd(),
    useEslintrc: Boolean(globPattern.length),
    baseConfig: {
      extends: [`taro/${projectConfig.framework}`]
    }
  })

  const sourceFiles = path.join(process.cwd(), projectConfig.sourceRoot, '**/*.{js,ts,jsx,tsx}')
  const report = await eslintCli.lintFiles([sourceFiles])
  const formatter = await eslintCli.loadFormatter()
  const rawReport = formatter.format(report)
  console.log(`\u{1F3AF} 检查 ESLint (以下为 ESLint 的输出)！`)
  if (rawReport) {
    console.log(rawReport)
  } else {
    console.log(`${chalk.green('[\u{2713}]')} Eslint 检查通过！`)
  }
}
