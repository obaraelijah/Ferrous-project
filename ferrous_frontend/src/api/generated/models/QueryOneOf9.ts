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
 * @interface QueryOneOf9
 */
export interface QueryOneOf9 {
    /**
     * 
     * @type {SearchType}
     * @memberof QueryOneOf9
     */
    address: SearchType;
}

/**
 * Check if a given object implements the QueryOneOf9 interface.
 */
export function instanceOfQueryOneOf9(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "address" in value;

    return isInstance;
}

export function QueryOneOf9FromJSON(json: any): QueryOneOf9 {
    return QueryOneOf9FromJSONTyped(json, false);
}

export function QueryOneOf9FromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryOneOf9 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'address': SearchTypeFromJSON(json['Address']),
    };
}

export function QueryOneOf9ToJSON(value?: QueryOneOf9 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Address': SearchTypeToJSON(value.address),
    };
}