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

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface FinishRegisterRequestAllOf
 */
export interface FinishRegisterRequestAllOf {
    /**
     * 
     * @type {string}
     * @memberof FinishRegisterRequestAllOf
     */
    name: string;
}

export function FinishRegisterRequestAllOfFromJSON(json: any): FinishRegisterRequestAllOf {
    return FinishRegisterRequestAllOfFromJSONTyped(json, false);
}

export function FinishRegisterRequestAllOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): FinishRegisterRequestAllOf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'name': json['name'],
    };
}

export function FinishRegisterRequestAllOfToJSON(value?: FinishRegisterRequestAllOf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'name': value.name,
    };
}

