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
  CreateAppRequest,
  FullOauthClient,
  GetAppsResponse,
  UpdateAppRequest,
  UuidResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateAppRequestFromJSON,
    CreateAppRequestToJSON,
    FullOauthClientFromJSON,
    FullOauthClientToJSON,
    GetAppsResponseFromJSON,
    GetAppsResponseToJSON,
    UpdateAppRequestFromJSON,
    UpdateAppRequestToJSON,
    UuidResponseFromJSON,
    UuidResponseToJSON,
} from '../models';

export interface CreateOauthAppRequest {
    createAppRequest: CreateAppRequest;
}

export interface DeleteOauthAppRequest {
    uuid: string;
}

export interface GetOauthAppRequest {
    uuid: string;
}

export interface UpdateOauthAppRequest {
    uuid: string;
    updateAppRequest: UpdateAppRequest;
}

/**
 * 
 */
export class OAuthApplicationApi extends runtime.BaseAPI {

    /**
     * Create a new application
     * Create a new application
     */
    async createOauthAppRaw(requestParameters: CreateOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.createAppRequest === null || requestParameters.createAppRequest === undefined) {
            throw new runtime.RequiredError('createAppRequest','Required parameter requestParameters.createAppRequest was null or undefined when calling createOauthApp.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/admin/applications`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateAppRequestToJSON(requestParameters.createAppRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Create a new application
     * Create a new application
     */
    async createOauthApp(requestParameters: CreateOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.createOauthAppRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete an application
     * Delete an application
     */
    async deleteOauthAppRaw(requestParameters: DeleteOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling deleteOauthApp.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/admin/applications/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Delete an application
     * Delete an application
     */
    async deleteOauthApp(requestParameters: DeleteOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteOauthAppRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getAllOauthAppsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<GetAppsResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/admin/applications`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GetAppsResponseFromJSON(jsonValue));
    }

    /**
     */
    async getAllOauthApps(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<GetAppsResponse> {
        const response = await this.getAllOauthAppsRaw(initOverrides);
        return await response.value();
    }

    /**
     */
    async getOauthAppRaw(requestParameters: GetOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullOauthClient>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getOauthApp.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/admin/applications/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullOauthClientFromJSON(jsonValue));
    }

    /**
     */
    async getOauthApp(requestParameters: GetOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullOauthClient> {
        const response = await this.getOauthAppRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Update an application
     * Update an application
     */
    async updateOauthAppRaw(requestParameters: UpdateOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling updateOauthApp.');
        }

        if (requestParameters.updateAppRequest === null || requestParameters.updateAppRequest === undefined) {
            throw new runtime.RequiredError('updateAppRequest','Required parameter requestParameters.updateAppRequest was null or undefined when calling updateOauthApp.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/admin/applications/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateAppRequestToJSON(requestParameters.updateAppRequest),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Update an application
     * Update an application
     */
    async updateOauthApp(requestParameters: UpdateOauthAppRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateOauthAppRaw(requestParameters, initOverrides);
    }

}
