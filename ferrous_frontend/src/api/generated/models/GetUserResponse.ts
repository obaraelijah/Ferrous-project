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
import type { GetUser } from './GetUser';
import {
    GetUserFromJSON,
    GetUserFromJSONTyped,
    GetUserToJSON,
} from './GetUser';

/**
 * 
 * @export
 * @interface GetUserResponse
 */
export interface GetUserResponse {
    /**
     * 
     * @type {Array<GetUser>}
     * @memberof GetUserResponse
     */
    users: Array<GetUser>;
}

/**
 * Check if a given object implements the GetUserResponse interface.
 */
export function instanceOfGetUserResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "users" in value;

    return isInstance;
}

export function GetUserResponseFromJSON(json: any): GetUserResponse {
    return GetUserResponseFromJSONTyped(json, false);
}

export function GetUserResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetUserResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'users': ((json['users'] as Array<any>).map(GetUserFromJSON)),
    };
}

export function GetUserResponseToJSON(value?: GetUserResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'users': ((value.users as Array<any>).map(GetUserToJSON)),
    };
}
