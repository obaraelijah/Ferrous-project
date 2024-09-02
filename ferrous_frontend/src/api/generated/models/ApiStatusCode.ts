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
 * @enum {string}
 */
export enum ApiStatusCode {
    NUMBER_1000 = 1000,
    NUMBER_1001 = 1001,
    NUMBER_1002 = 1002,
    NUMBER_1003 = 1003,
    NUMBER_1004 = 1004,
    NUMBER_1005 = 1005,
    NUMBER_1006 = 1006,
    NUMBER_1007 = 1007,
    NUMBER_1008 = 1008,
    NUMBER_1009 = 1009,
    NUMBER_1010 = 1010,
    NUMBER_1011 = 1011,
    NUMBER_1012 = 1012,
    NUMBER_1013 = 1013,
    NUMBER_1014 = 1014,
    NUMBER_1015 = 1015,
    NUMBER_1016 = 1016,
    NUMBER_1017 = 1017,
    NUMBER_1018 = 1018,
    NUMBER_1019 = 1019,
    NUMBER_2000 = 2000,
    NUMBER_2001 = 2001,
    NUMBER_2002 = 2002,
    NUMBER_2003 = 2003
}

export function ApiStatusCodeFromJSON(json: any): ApiStatusCode {
    return ApiStatusCodeFromJSONTyped(json, false);
}

export function ApiStatusCodeFromJSONTyped(json: any, ignoreDiscriminator: boolean): ApiStatusCode {
    return json as ApiStatusCode;
}

export function ApiStatusCodeToJSON(value?: ApiStatusCode | null): any {
    return value as any;
}

