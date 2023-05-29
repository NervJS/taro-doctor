import {
  validateEnv,
  validateConfig,
  validatePackage,
  validateRecommend,
  validateEslint
} from './js-binding'

export default (ctx) => {
	ctx.registerCommand({
    name: 'dd',
    fn() {
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
      validateEslint()
    },
  })
}
