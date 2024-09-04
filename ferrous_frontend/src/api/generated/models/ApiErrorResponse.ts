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
import type { ApiStatusCode } from './ApiStatusCode';
import {
    ApiStatusCodeFromJSON,
    ApiStatusCodeFromJSONTyped,
    ApiStatusCodeToJSON,
} from './ApiStatusCode';

/**
 * Representation of an error response
 * * `status_code` holds the error code, `message` a human readable description of the error
 * @export
 * @interface ApiErrorResponse
 */
export interface ApiErrorResponse {
    /**
     * 
     * @type {ApiStatusCode}
     * @memberof ApiErrorResponse
     */
    statusCode: ApiStatusCode;
    /**
     * 
     * @type {string}
     * @memberof ApiErrorResponse
     */
    message: string;
}

/**
 * Check if a given object implements the ApiErrorResponse interface.
 */
export function instanceOfApiErrorResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "statusCode" in value;
    isInstance = isInstance && "message" in value;

    return isInstance;
}

export function ApiErrorResponseFromJSON(json: any): ApiErrorResponse {
    return ApiErrorResponseFromJSONTyped(json, false);
}

export function ApiErrorResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ApiErrorResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'statusCode': ApiStatusCodeFromJSON(json['status_code']),
        'message': json['message'],
    };
}

export function ApiErrorResponseToJSON(value?: ApiErrorResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'status_code': ApiStatusCodeToJSON(value.statusCode),
        'message': value.message,
    };
}
