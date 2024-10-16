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
 * @interface SearchTypeOneOf4
 */
export interface SearchTypeOneOf4 {
    /**
     * Add multiple [SearchType]s with an AND
     * @type {Array<SearchType>}
     * @memberof SearchTypeOneOf4
     */
    and: Array<SearchType>;
}

/**
 * Check if a given object implements the SearchTypeOneOf4 interface.
 */
export function instanceOfSearchTypeOneOf4(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "and" in value;

    return isInstance;
}

export function SearchTypeOneOf4FromJSON(json: any): SearchTypeOneOf4 {
    return SearchTypeOneOf4FromJSONTyped(json, false);
}

export function SearchTypeOneOf4FromJSONTyped(json: any, ignoreDiscriminator: boolean): SearchTypeOneOf4 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'and': ((json['And'] as Array<any>).map(SearchTypeFromJSON)),
    };
}

export function SearchTypeOneOf4ToJSON(value?: SearchTypeOneOf4 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'And': ((value.and as Array<any>).map(SearchTypeToJSON)),
    };
}
