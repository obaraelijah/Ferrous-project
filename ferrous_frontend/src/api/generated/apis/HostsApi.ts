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
  FullHost,
  HostResultsPage,
  UpdateHostRequest,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    FullHostFromJSON,
    FullHostToJSON,
    HostResultsPageFromJSON,
    HostResultsPageToJSON,
    UpdateHostRequestFromJSON,
    UpdateHostRequestToJSON,
} from '../models';

export interface GetAllHostsRequest {
    uuid: string;
    limit: number;
    offset: number;
}

export interface GetHostRequest {
    wUuid: string;
    hUuid: string;
}

export interface UpdateHostOperationRequest {
    wUuid: string;
    hUuid: string;
    updateHostRequest: UpdateHostRequest;
}

/**
 * 
 */
export class HostsApi extends runtime.BaseAPI {

    /**
     * Retrieve all hosts.  Hosts are created out of aggregating data or by user input. They represent a single host and can be created by providing an IP address
     * Retrieve all hosts.
     */
    async getAllHostsRaw(requestParameters: GetAllHostsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<HostResultsPage>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAllHosts.');
        }

        if (requestParameters.limit === null || requestParameters.limit === undefined) {
            throw new runtime.RequiredError('limit','Required parameter requestParameters.limit was null or undefined when calling getAllHosts.');
        }

        if (requestParameters.offset === null || requestParameters.offset === undefined) {
            throw new runtime.RequiredError('offset','Required parameter requestParameters.offset was null or undefined when calling getAllHosts.');
        }

        const queryParameters: any = {};

        if (requestParameters.limit !== undefined) {
            queryParameters['limit'] = requestParameters.limit;
        }

        if (requestParameters.offset !== undefined) {
            queryParameters['offset'] = requestParameters.offset;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/hosts`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => HostResultsPageFromJSON(jsonValue));
    }

    /**
     * Retrieve all hosts.  Hosts are created out of aggregating data or by user input. They represent a single host and can be created by providing an IP address
     * Retrieve all hosts.
     */
    async getAllHosts(requestParameters: GetAllHostsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<HostResultsPage> {
        const response = await this.getAllHostsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Retrieve all information about a single host
     * Retrieve all information about a single host
     */
    async getHostRaw(requestParameters: GetHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullHost>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling getHost.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling getHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts{h_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullHostFromJSON(jsonValue));
    }

    /**
     * Retrieve all information about a single host
     * Retrieve all information about a single host
     */
    async getHost(requestParameters: GetHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullHost> {
        const response = await this.getHostRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Update a host  You must include at least on parameter
     * Update a host
     */
    async updateHostRaw(requestParameters: UpdateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling updateHost.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling updateHost.');
        }

        if (requestParameters.updateHostRequest === null || requestParameters.updateHostRequest === undefined) {
            throw new runtime.RequiredError('updateHostRequest','Required parameter requestParameters.updateHostRequest was null or undefined when calling updateHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateHostRequestToJSON(requestParameters.updateHostRequest),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Update a host  You must include at least on parameter
     * Update a host
     */
    async updateHost(requestParameters: UpdateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateHostRaw(requestParameters, initOverrides);
    }

}