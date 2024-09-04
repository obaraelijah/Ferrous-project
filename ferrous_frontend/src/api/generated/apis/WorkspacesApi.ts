/* tslint:disable */
/* eslint-disable */
/**
 * ferrrous
 * The core component of ferrrous-project
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
    CreateWorkspaceRequest,
    CreateWorkspaceResponse,
    GetWorkspace,
    GetWorkspaceResponse,
    UpdateWorkspaceRequest,
  } from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateWorkspaceRequestFromJSON,
    CreateWorkspaceRequestToJSON,
    CreateWorkspaceResponseFromJSON,
    CreateWorkspaceResponseToJSON,
    GetWorkspaceFromJSON,
    GetWorkspaceToJSON,
    GetWorkspaceResponseFromJSON,
    GetWorkspaceResponseToJSON,
    UpdateWorkspaceRequestFromJSON,
    UpdateWorkspaceRequestToJSON,
} from '../models';

export interface CreateWorkspaceOperationRequest {
    createWorkspaceRequest: CreateWorkspaceRequest;
}

export interface DeleteWorkspaceRequest {
    id: number;
}

export interface GetWorkspaceRequest {
    id: number;
}

export interface UpdateWorkspaceOperationRequest {
    id: number;
    updateWorkspaceRequest: UpdateWorkspaceRequest;
}

/**
 * 
 */
export class WorkspacesApi extends runtime.BaseAPI {

    /**
     * Create a new workspace
     * Create a new workspace
     */
    async createWorkspaceRaw(requestParameters: CreateWorkspaceOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<CreateWorkspaceResponse>> {
        if (requestParameters.createWorkspaceRequest === null || requestParameters.createWorkspaceRequest === undefined) {
            throw new runtime.RequiredError('createWorkspaceRequest','Required parameter requestParameters.createWorkspaceRequest was null or undefined when calling createWorkspace.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateWorkspaceRequestToJSON(requestParameters.createWorkspaceRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => CreateWorkspaceResponseFromJSON(jsonValue));
    }

    /**
     * Create a new workspace
     * Create a new workspace
     */
    async createWorkspace(requestParameters: CreateWorkspaceOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<CreateWorkspaceResponse> {
        const response = await this.createWorkspaceRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete a workspace by its id
     * Delete a workspace by its id
     */
    async deleteWorkspaceRaw(requestParameters: DeleteWorkspaceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling deleteWorkspace.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Delete a workspace by its id
     * Delete a workspace by its id
     */
    async deleteWorkspace(requestParameters: DeleteWorkspaceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteWorkspaceRaw(requestParameters, initOverrides);
    }

    /**
     * Retrieve all workspaces owned by executing user  For administration access, look at the `/admin/workspaces` endpoint.
     * Retrieve all workspaces owned by executing user
     */
    async getAllWorkspacesRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<GetWorkspaceResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GetWorkspaceResponseFromJSON(jsonValue));
    }

    /**
     * Retrieve all workspaces owned by executing user  For administration access, look at the `/admin/workspaces` endpoint.
     * Retrieve all workspaces owned by executing user
     */
    async getAllWorkspaces(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<GetWorkspaceResponse> {
        const response = await this.getAllWorkspacesRaw(initOverrides);
        return await response.value();
    }

    /**
     * Retrieve a workspace by id
     * Retrieve a workspace by id
     */
    async getWorkspaceRaw(requestParameters: GetWorkspaceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<GetWorkspace>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getWorkspace.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GetWorkspaceFromJSON(jsonValue));
    }

    /**
     * Retrieve a workspace by id
     * Retrieve a workspace by id
     */
    async getWorkspace(requestParameters: GetWorkspaceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<GetWorkspace> {
        const response = await this.getWorkspaceRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Updates a workspace by its id  All parameter are optional, but at least one of them must be specified.  `name` must not be empty.  You can set `description` to null to remove the description from the database. If you leave the parameter out, the description will remain unchanged.
     * Updates a workspace by its id
     */
    async updateWorkspaceRaw(requestParameters: UpdateWorkspaceOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling updateWorkspace.');
        }

        if (requestParameters.updateWorkspaceRequest === null || requestParameters.updateWorkspaceRequest === undefined) {
            throw new runtime.RequiredError('updateWorkspaceRequest','Required parameter requestParameters.updateWorkspaceRequest was null or undefined when calling updateWorkspace.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateWorkspaceRequestToJSON(requestParameters.updateWorkspaceRequest),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Updates a workspace by its id  All parameter are optional, but at least one of them must be specified.  `name` must not be empty.  You can set `description` to null to remove the description from the database. If you leave the parameter out, the description will remain unchanged.
     * Updates a workspace by its id
     */
    async updateWorkspace(requestParameters: UpdateWorkspaceOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateWorkspaceRaw(requestParameters, initOverrides);
    }
}