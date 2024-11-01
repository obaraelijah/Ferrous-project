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
 * Request to create a new api key
 * @export
 * @interface CreateApiKeyRequest
 */
export interface CreateApiKeyRequest {
    /**
     * A descriptive name helping the user to identify the key
     * @type {string}
     * @memberof CreateApiKeyRequest
     */
    name: string;
}

/**
 * Check if a given object implements the CreateApiKeyRequest interface.
 */
export function instanceOfCreateApiKeyRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "name" in value;

    return isInstance;
}

export function CreateApiKeyRequestFromJSON(json: any): CreateApiKeyRequest {
    return CreateApiKeyRequestFromJSONTyped(json, false);
}

export function CreateApiKeyRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateApiKeyRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
    };
}

export function CreateApiKeyRequestToJSON(value?: CreateApiKeyRequest | null): any {
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
