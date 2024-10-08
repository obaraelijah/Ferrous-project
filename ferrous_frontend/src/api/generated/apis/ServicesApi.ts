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


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  GetAllServicesResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    GetAllServicesResponseFromJSON,
    GetAllServicesResponseToJSON,
} from '../models';

export interface GetAllServicesRequest {
    uuid: string;
}

/**
 * 
 */
export class ServicesApi extends runtime.BaseAPI {

    /**
     * List the services of a workspace
     * List the services of a workspace
     */
    async getAllServicesRaw(requestParameters: GetAllServicesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<GetAllServicesResponse>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAllServices.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/services`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GetAllServicesResponseFromJSON(jsonValue));
    }

    /**
     * List the services of a workspace
     * List the services of a workspace
     */
    async getAllServices(requestParameters: GetAllServicesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<GetAllServicesResponse> {
        const response = await this.getAllServicesRaw(requestParameters, initOverrides);
        return await response.value();
    }

}