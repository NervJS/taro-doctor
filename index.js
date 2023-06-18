"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.validateEslintPrint = exports.validateEslint = exports.validateRecommendPrint = exports.validateRecommend = exports.validatePackagePrint = exports.validatePackage = exports.validateEnvPrint = exports.validateEnv = exports.validateConfigPrint = exports.validateConfig = void 0;
const path = require("path");
const eslint_1 = require("eslint");
const glob = require("glob");
const js_binding_1 = require("./js-binding");
exports.default = (ctx) => {
    ctx.registerCommand({
        name: 'doctor',
        fn() {
            return __awaiter(this, void 0, void 0, function* () {
                const { appPath, nodeModulesPath, configPath } = ctx.paths;
                const { fs, chalk, PROJECT_CONFIG } = ctx.helper;
                if (!configPath || !fs.existsSync(configPath)) {
                    console.log(chalk.red(`找不到项目配置文件${PROJECT_CONFIG}，请确定当前目录是 Taro 项目根目录!`));
                    process.exit(1);
                }
                validateEnvPrint();
                yield validateConfigPrint(ctx.initialConfig, ctx.helper);
                validatePackagePrint(appPath, nodeModulesPath);
                validateRecommendPrint(appPath);
                yield validateEslintPrint(ctx.initialConfig, chalk);
            });
        },
    });
};
function getValidateConfigParams(projectConfig, helper) {
    return __awaiter(this, void 0, void 0, function* () {
        const configStr = JSON.stringify(projectConfig, (_, v) => {
            if (typeof v === 'function') {
                return '__function__';
            }
            return v;
        });
        let remoteSchemaUrl = 'https://raw.githubusercontent.com/NervJS/taro-doctor/main/assets/config_schema.json';
        let useRemoteSchema = true;
        const homedir = helper.getUserHomeDir();
        if (homedir) {
            const taroConfigPath = path.join(homedir, helper.TARO_CONFIG_FOLDER);
            const taroConfig = path.join(taroConfigPath, helper.TARO_BASE_CONFIG);
            if (helper.fs.existsSync(taroConfig)) {
                const config = yield helper.fs.readJSON(taroConfig);
                remoteSchemaUrl = config && config.remoteConfigSchemaUrl ? config.remoteConfigSchemaUrl : remoteSchemaUrl;
                useRemoteSchema = config && config.useRemoteConfigSchema ? config.useRemoteConfigSchema : useRemoteSchema;
            }
            else {
                yield helper.fs.createFile(taroConfig);
                yield helper.fs.writeJSON(taroConfig, { remoteSchemaUrl, useRemoteSchema });
            }
        }
        return { configStr, remoteSchemaUrl, useRemoteSchema };
    });
}
function validateConfig(projectConfig, helper) {
    return __awaiter(this, void 0, void 0, function* () {
        const { configStr, remoteSchemaUrl, useRemoteSchema } = yield getValidateConfigParams(projectConfig, helper);
        return (0, js_binding_1.validateConfig)(configStr, remoteSchemaUrl, useRemoteSchema);
    });
}
exports.validateConfig = validateConfig;
function validateConfigPrint(projectConfig, helper) {
    return __awaiter(this, void 0, void 0, function* () {
        const { configStr, remoteSchemaUrl, useRemoteSchema } = yield getValidateConfigParams(projectConfig, helper);
        return (0, js_binding_1.validateConfigPrint)(configStr, remoteSchemaUrl, useRemoteSchema);
    });
}
exports.validateConfigPrint = validateConfigPrint;
function validateEnv() {
    return (0, js_binding_1.validateEnv)();
}
exports.validateEnv = validateEnv;
function validateEnvPrint() {
    return (0, js_binding_1.validateEnvPrint)();
}
exports.validateEnvPrint = validateEnvPrint;
function validatePackage(appPath, nodeModulesPath) {
    return (0, js_binding_1.validatePackage)(appPath, nodeModulesPath);
}
exports.validatePackage = validatePackage;
function validatePackagePrint(appPath, nodeModulesPath) {
    return (0, js_binding_1.validatePackagePrint)(appPath, nodeModulesPath);
}
exports.validatePackagePrint = validatePackagePrint;
function validateRecommend(appPath) {
    return (0, js_binding_1.validateRecommend)(appPath);
}
exports.validateRecommend = validateRecommend;
function validateRecommendPrint(appPath) {
    return (0, js_binding_1.validateRecommendPrint)(appPath);
}
exports.validateRecommendPrint = validateRecommendPrint;
function validateEslint(projectConfig, chalk) {
    return __awaiter(this, void 0, void 0, function* () {
        const result = yield validateEslintCore(projectConfig, chalk);
        result.messages.unshift({
            kind: 0 /* MessageKind.Info */,
            content: `\u{1F3AF} 检查 ESLint (以下为 ESLint 的输出)！`,
        });
        return result;
    });
}
exports.validateEslint = validateEslint;
function validateEslintPrint(projectConfig, chalk) {
    return __awaiter(this, void 0, void 0, function* () {
        const result = yield validateEslintCore(projectConfig, chalk);
        let is_valid = result.isValid;
        let rawReport = result.messages[0].content;
        console.log(`\u{1F3AF} 检查 ESLint (以下为 ESLint 的输出)！`);
        if (is_valid) {
            console.log(`${chalk.green('[\u{2713}]')} Eslint 检查通过！`);
        }
        else {
            console.log(rawReport);
        }
        return is_valid;
    });
}
exports.validateEslintPrint = validateEslintPrint;
function validateEslintCore(projectConfig, chalk) {
    return __awaiter(this, void 0, void 0, function* () {
        const appPath = process.cwd();
        const globPattern = glob.sync(path.join(appPath, '.eslintrc*'));
        const eslintCli = new eslint_1.ESLint({
            cwd: process.cwd(),
            useEslintrc: Boolean(globPattern.length),
            baseConfig: {
                extends: [`taro/${projectConfig.framework}`],
            },
        });
        const sourceFiles = path.join(process.cwd(), projectConfig.sourceRoot, '**/*.{js,ts,jsx,tsx}');
        const report = yield eslintCli.lintFiles([sourceFiles]);
        const formatter = yield eslintCli.loadFormatter();
        let rawReport = formatter.format(report);
        let is_valid = true;
        if (rawReport) {
            is_valid = false;
        }
        if (is_valid) {
            rawReport = `${chalk.green('[\u{2713}]')} Eslint 检查通过！`;
        }
        return {
            isValid: is_valid,
            messages: [
                {
                    kind: is_valid ? 2 /* MessageKind.Success */ : 1 /* MessageKind.Error */,
                    content: rawReport,
                },
            ],
        };
    });
}
