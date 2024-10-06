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
import type { Color } from './Color';
import {
    ColorFromJSON,
    ColorFromJSONTyped,
    ColorToJSON,
} from './Color';

/**
 * The request to create a workspace tag
 * @export
 * @interface CreateWorkspaceTagRequest
 */
export interface CreateWorkspaceTagRequest {
    /**
     * Name of the tag
     * @type {string}
     * @memberof CreateWorkspaceTagRequest
     */
    name: string;
    /**
     * 
     * @type {Color}
     * @memberof CreateWorkspaceTagRequest
     */
    color: Color;
}

/**
 * Check if a given object implements the CreateWorkspaceTagRequest interface.
 */
export function instanceOfCreateWorkspaceTagRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "color" in value;

    return isInstance;
}

export function CreateWorkspaceTagRequestFromJSON(json: any): CreateWorkspaceTagRequest {
    return CreateWorkspaceTagRequestFromJSONTyped(json, false);
}

export function CreateWorkspaceTagRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateWorkspaceTagRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'color': ColorFromJSON(json['color']),
    };
}

export function CreateWorkspaceTagRequestToJSON(value?: CreateWorkspaceTagRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'color': ColorToJSON(value.color),
    };
}
