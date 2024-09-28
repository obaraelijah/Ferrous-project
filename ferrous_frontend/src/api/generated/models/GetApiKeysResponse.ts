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
import type { SimpleApiKey } from './SimpleApiKey';
import {
    SimpleApiKeyFromJSON,
    SimpleApiKeyFromJSONTyped,
    SimpleApiKeyToJSON,
} from './SimpleApiKey';

/**
 * 
 * @export
 * @interface GetApiKeysResponse
 */
export interface GetApiKeysResponse {
    /**
     * 
     * @type {Array<SimpleApiKey>}
     * @memberof GetApiKeysResponse
     */
    keys: Array<SimpleApiKey>;
}

/**
 * Check if a given object implements the GetApiKeysResponse interface.
 */
export function instanceOfGetApiKeysResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "keys" in value;

    return isInstance;
}

export function GetApiKeysResponseFromJSON(json: any): GetApiKeysResponse {
    return GetApiKeysResponseFromJSONTyped(json, false);
}

export function GetApiKeysResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetApiKeysResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'keys': ((json['keys'] as Array<any>).map(SimpleApiKeyFromJSON)),
    };
}

export function GetApiKeysResponseToJSON(value?: GetApiKeysResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'keys': ((value.keys as Array<any>).map(SimpleApiKeyToJSON)),
    };
}
