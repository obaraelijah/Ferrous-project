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
 * @interface TokenErrorResponse
 */
export interface TokenErrorResponse {
    /**
     * 
     * @type {string}
     * @memberof TokenErrorResponse
     */
    error: string;
}

/**
 * Check if a given object implements the TokenErrorResponse interface.
 */
export function instanceOfTokenErrorResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "error" in value;

    return isInstance;
}

export function TokenErrorResponseFromJSON(json: any): TokenErrorResponse {
    return TokenErrorResponseFromJSONTyped(json, false);
}

export function TokenErrorResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): TokenErrorResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'error': json['error'],
    };
}

export function TokenErrorResponseToJSON(value?: TokenErrorResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'error': value.error,
    };
}
