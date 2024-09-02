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
 * The response of an attack
 * @export
 * @interface AttackResponse
 */
export interface AttackResponse {
    /**
     * 
     * @type {number}
     * @memberof AttackResponse
     */
    attackId: number;
}

export function AttackResponseFromJSON(json: any): AttackResponse {
    return AttackResponseFromJSONTyped(json, false);
}

export function AttackResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): AttackResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'attackId': json['attack_id'],
    };
}

export function AttackResponseToJSON(value?: AttackResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'attack_id': value.attackId,
    };
}
