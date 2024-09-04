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
 * @interface UpdateLeechRequest
 */
export interface UpdateLeechRequest {
    /**
     * 
     * @type {string}
     * @memberof UpdateLeechRequest
     */
    name?: string | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateLeechRequest
     */
    address?: string | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateLeechRequest
     */
    description?: string | null;
}

/**
 * Check if a given object implements the UpdateLeechRequest interface.
 */
export function instanceOfUpdateLeechRequest(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function UpdateLeechRequestFromJSON(json: any): UpdateLeechRequest {
    return UpdateLeechRequestFromJSONTyped(json, false);
}

export function UpdateLeechRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): UpdateLeechRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'name': !exists(json, 'name') ? undefined : json['name'],
        'address': !exists(json, 'address') ? undefined : json['address'],
        'description': !exists(json, 'description') ? undefined : json['description'],
    };
}

export function UpdateLeechRequestToJSON(value?: UpdateLeechRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'name': value.name,
        'address': value.address,
        'description': value.description,
    };
}
