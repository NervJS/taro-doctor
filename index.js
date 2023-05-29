"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var js_binding_1 = require("./js-binding");
exports.default = (function (ctx) {
    ctx.registerCommand({
        name: 'dd',
        fn: function () {
            var _a = ctx.paths, appPath = _a.appPath, nodeModulesPath = _a.nodeModulesPath, configPath = _a.configPath;
            var _b = ctx.helper, fs = _b.fs, chalk = _b.chalk, PROJECT_CONFIG = _b.PROJECT_CONFIG;
            if (!configPath || !fs.existsSync(configPath)) {
                console.log(chalk.red("\u627E\u4E0D\u5230\u9879\u76EE\u914D\u7F6E\u6587\u4EF6".concat(PROJECT_CONFIG, "\uFF0C\u8BF7\u786E\u5B9A\u5F53\u524D\u76EE\u5F55\u662F Taro \u9879\u76EE\u6839\u76EE\u5F55!")));
                process.exit(1);
            }
            var configStr = JSON.stringify(ctx.initialConfig, function (_, v) {
                if (typeof v === 'function') {
                    return '__function__';
                }
                return v;
            });
            (0, js_binding_1.validateEnv)();
            (0, js_binding_1.validateConfig)(configStr);
            (0, js_binding_1.validatePackage)(appPath, nodeModulesPath);
            (0, js_binding_1.validateRecommend)(appPath);
            (0, js_binding_1.validateEslint)();
        },
    });
});
