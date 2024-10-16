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
import type { SearchType } from './SearchType';
import {
    SearchTypeFromJSON,
    SearchTypeFromJSONTyped,
    SearchTypeToJSON,
} from './SearchType';

/**
 * 
 * @export
 * @interface QueryOneOf3
 */
export interface QueryOneOf3 {
    /**
     * 
     * @type {SearchType}
     * @memberof QueryOneOf3
     */
    password: SearchType;
}

/**
 * Check if a given object implements the QueryOneOf3 interface.
 */
export function instanceOfQueryOneOf3(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "password" in value;

    return isInstance;
}

export function QueryOneOf3FromJSON(json: any): QueryOneOf3 {
    return QueryOneOf3FromJSONTyped(json, false);
}

export function QueryOneOf3FromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryOneOf3 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'password': SearchTypeFromJSON(json['Password']),
    };
}

export function QueryOneOf3ToJSON(value?: QueryOneOf3 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Password': SearchTypeToJSON(value.password),
    };
}
