/* tslint:disable */
/* eslint-disable */
/**
 * ferrous
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


/**
 * 
 * @export
 */
export const CodeChallengeMethod = {
    S256: 'S256',
    Plain: 'plain'
} as const;
export type CodeChallengeMethod = typeof CodeChallengeMethod[keyof typeof CodeChallengeMethod];


export function CodeChallengeMethodFromJSON(json: any): CodeChallengeMethod {
    return CodeChallengeMethodFromJSONTyped(json, false);
}

export function CodeChallengeMethodFromJSONTyped(json: any, ignoreDiscriminator: boolean): CodeChallengeMethod {
    return json as CodeChallengeMethod;
}

export function CodeChallengeMethodToJSON(value?: CodeChallengeMethod | null): any {
    return value as any;
}
