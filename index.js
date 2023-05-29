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
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var eslint_1 = require("eslint");
var glob = require("glob");
var js_binding_1 = require("./js-binding");
exports.default = (function (ctx) {
    ctx.registerCommand({
        name: 'dd',
        fn: function () {
            return __awaiter(this, void 0, void 0, function () {
                var _a, appPath, nodeModulesPath, configPath, _b, fs, chalk, PROJECT_CONFIG, configStr;
                return __generator(this, function (_c) {
                    switch (_c.label) {
                        case 0:
                            _a = ctx.paths, appPath = _a.appPath, nodeModulesPath = _a.nodeModulesPath, configPath = _a.configPath;
                            _b = ctx.helper, fs = _b.fs, chalk = _b.chalk, PROJECT_CONFIG = _b.PROJECT_CONFIG;
                            if (!configPath || !fs.existsSync(configPath)) {
                                console.log(chalk.red("\u627E\u4E0D\u5230\u9879\u76EE\u914D\u7F6E\u6587\u4EF6".concat(PROJECT_CONFIG, "\uFF0C\u8BF7\u786E\u5B9A\u5F53\u524D\u76EE\u5F55\u662F Taro \u9879\u76EE\u6839\u76EE\u5F55!")));
                                process.exit(1);
                            }
                            configStr = JSON.stringify(ctx.initialConfig, function (_, v) {
                                if (typeof v === 'function') {
                                    return '__function__';
                                }
                                return v;
                            });
                            (0, js_binding_1.validateEnv)();
                            (0, js_binding_1.validateConfig)(configStr);
                            (0, js_binding_1.validatePackage)(appPath, nodeModulesPath);
                            (0, js_binding_1.validateRecommend)(appPath);
                            return [4 /*yield*/, validateEslint(ctx.initialConfig, chalk)];
                        case 1:
                            _c.sent();
                            return [2 /*return*/];
                    }
                });
            });
        },
    });
});
function validateEslint(projectConfig, chalk) {
    return __awaiter(this, void 0, void 0, function () {
        var appPath, globPattern, eslintCli, sourceFiles, report, formatter, rawReport;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    appPath = process.cwd();
                    globPattern = glob.sync(path.join(appPath, '.eslintrc*'));
                    eslintCli = new eslint_1.ESLint({
                        cwd: process.cwd(),
                        useEslintrc: Boolean(globPattern.length),
                        baseConfig: {
                            extends: ["taro/".concat(projectConfig.framework)]
                        }
                    });
                    sourceFiles = path.join(process.cwd(), projectConfig.sourceRoot, '**/*.{js,ts,jsx,tsx}');
                    return [4 /*yield*/, eslintCli.lintFiles([sourceFiles])];
                case 1:
                    report = _a.sent();
                    return [4 /*yield*/, eslintCli.loadFormatter()];
                case 2:
                    formatter = _a.sent();
                    rawReport = formatter.format(report);
                    console.log("\uD83C\uDFAF \u68C0\u67E5 ESLint (\u4EE5\u4E0B\u4E3A ESLint \u7684\u8F93\u51FA)\uFF01");
                    if (rawReport) {
                        console.log(rawReport);
                    }
                    else {
                        console.log("".concat(chalk.green("[\u2713]"), " Eslint \u68C0\u67E5\u901A\u8FC7\uFF01"));
                    }
                    return [2 /*return*/];
            }
        });
    });
}
