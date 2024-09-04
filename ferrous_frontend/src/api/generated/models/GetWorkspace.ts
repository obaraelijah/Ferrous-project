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
import type { UserResponse } from './UserResponse';
import {
    UserResponseFromJSON,
    UserResponseFromJSONTyped,
    UserResponseToJSON,
} from './UserResponse';

/**
 * 
 * @export
 * @interface GetWorkspace
 */
export interface GetWorkspace {
    /**
     * 
     * @type {number}
     * @memberof GetWorkspace
     */
    id: number;
    /**
     * 
     * @type {string}
     * @memberof GetWorkspace
     */
    description?: string | null;
    /**
     * 
     * @type {string}
     * @memberof GetWorkspace
     */
    name: string;
    /**
     * 
     * @type {UserResponse}
     * @memberof GetWorkspace
     */
    owner: UserResponse;
}

/**
 * Check if a given object implements the GetWorkspace interface.
 */
export function instanceOfGetWorkspace(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "id" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "owner" in value;

    return isInstance;
}


export function GetWorkspaceFromJSON(json: any): GetWorkspace {
    return GetWorkspaceFromJSONTyped(json, false);
}

export function GetWorkspaceFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetWorkspace {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'id': json['id'],
        'name': json['name'],
        'description': !exists(json, 'description') ? undefined : json['description'],
        'owner': UserResponseFromJSON(json['owner']),
    };
}

export function GetWorkspaceToJSON(value?: GetWorkspace | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'id': value.id,
        'name': value.name,
        'description': value.description,
        'owner': UserResponseToJSON(value.owner),
    };
}