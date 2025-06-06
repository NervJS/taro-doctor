import { Message, MessageKind, ValidateResult } from './js-binding';
declare const _default: (ctx: any) => void;
export default _default;
export declare function validateConfig(projectConfig: any, helper: any): Promise<ValidateResult>;
export declare function validateConfigPrint(projectConfig: any, helper: any): Promise<boolean>;
export declare function validateEnv(): ValidateResult;
export declare function validateEnvPrint(): boolean;
export declare function validatePackage(appPath: string, nodeModulesPath: string): ValidateResult;
export declare function validatePackagePrint(appPath: string, nodeModulesPath: string): boolean;
export declare function validateRecommend(appPath: string): ValidateResult;
export declare function validateRecommendPrint(appPath: string): boolean;
export declare function validateEslint(projectConfig: any, chalk: any): Promise<ValidateResult>;
export declare function validateEslintPrint(projectConfig: any, chalk: any): Promise<boolean>;
export declare function validateStylelintPrint(projectConfig: any, chalk: any): Promise<boolean>;
export { Message, MessageKind, ValidateResult };
