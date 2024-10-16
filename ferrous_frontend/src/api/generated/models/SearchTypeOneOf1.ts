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
 * @interface SearchTypeOneOf1
 */
export interface SearchTypeOneOf1 {
    /**
     * Search for an exact pattern
     * @type {string}
     * @memberof SearchTypeOneOf1
     */
    exact: string;
}

/**
 * Check if a given object implements the SearchTypeOneOf1 interface.
 */
export function instanceOfSearchTypeOneOf1(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "exact" in value;

    return isInstance;
}

export function SearchTypeOneOf1FromJSON(json: any): SearchTypeOneOf1 {
    return SearchTypeOneOf1FromJSONTyped(json, false);
}

export function SearchTypeOneOf1FromJSONTyped(json: any, ignoreDiscriminator: boolean): SearchTypeOneOf1 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'exact': json['Exact'],
    };
}

export function SearchTypeOneOf1ToJSON(value?: SearchTypeOneOf1 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Exact': value.exact,
    };
}