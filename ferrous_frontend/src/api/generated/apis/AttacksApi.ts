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
    AttackResponse,
    BruteforceSubdomainsRequest,
    QueryCertificateTransparencyRequest,
    ScanTcpPortsRequest,
    SimpleAttack,
    TcpPortScanResultsPage,
  } from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    AttackResponseFromJSON,
    AttackResponseToJSON,
    BruteforceSubdomainsRequestFromJSON,
    BruteforceSubdomainsRequestToJSON,
    QueryCertificateTransparencyRequestFromJSON,
    QueryCertificateTransparencyRequestToJSON,
    ScanTcpPortsRequestFromJSON,
    ScanTcpPortsRequestToJSON,
    SimpleAttackFromJSON,
    SimpleAttackToJSON,
    TcpPortScanResultsPageFromJSON,
    TcpPortScanResultsPageToJSON,
} from '../models';

export interface BruteforceSubdomainsOperationRequest {
    bruteforceSubdomainsRequest: BruteforceSubdomainsRequest;
}

export interface DeleteAttackRequest {
    id: number;
}

export interface GetAttackRequest {
    id: number;
}

export interface GetTcpPortScanResultsRequest {
    id: number;
    limit: number;
    offset: number;
}

export interface QueryCertificateTransparencyOperationRequest {
    queryCertificateTransparencyRequest: QueryCertificateTransparencyRequest;
}

export interface ScanTcpPortsOperationRequest {
    scanTcpPortsRequest: ScanTcpPortsRequest;
}

/**
 * 
 */
export class AttacksApi extends runtime.BaseAPI {

    /**
     * Bruteforce subdomains through a DNS wordlist attack  Enumerate possible subdomains by querying a DNS server with constructed domains. See [OWASP](https://owasp.org/www-community/attacks/Brute_force_attack) for further information.
     * Bruteforce subdomains through a DNS wordlist attack
     */
    async bruteforceSubdomainsRaw(requestParameters: BruteforceSubdomainsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AttackResponse>> {
        if (requestParameters.bruteforceSubdomainsRequest === null || requestParameters.bruteforceSubdomainsRequest === undefined) {
            throw new runtime.RequiredError('bruteforceSubdomainsRequest','Required parameter requestParameters.bruteforceSubdomainsRequest was null or undefined when calling bruteforceSubdomains.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/bruteforceSubdomains`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: BruteforceSubdomainsRequestToJSON(requestParameters.bruteforceSubdomainsRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AttackResponseFromJSON(jsonValue));
    }

    /**
     * Bruteforce subdomains through a DNS wordlist attack  Enumerate possible subdomains by querying a DNS server with constructed domains. See [OWASP](https://owasp.org/www-community/attacks/Brute_force_attack) for further information.
     * Bruteforce subdomains through a DNS wordlist attack
     */
    async bruteforceSubdomains(requestParameters: BruteforceSubdomainsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AttackResponse> {
        const response = await this.bruteforceSubdomainsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete an attack and its results
     * Delete an attack and its results
     */
    async deleteAttackRaw(requestParameters: DeleteAttackRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling deleteAttack.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/attacks/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Delete an attack and its results
     * Delete an attack and its results
     */
    async deleteAttack(requestParameters: DeleteAttackRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteAttackRaw(requestParameters, initOverrides);
    }

    /**
     * Retrieve an attack by id
     * Retrieve an attack by id
     */
    async getAttackRaw(requestParameters: GetAttackRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<SimpleAttack>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getAttack.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/attacks/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SimpleAttackFromJSON(jsonValue));
    }

    /**
     * Retrieve an attack by id
     * Retrieve an attack by id
     */
    async getAttack(requestParameters: GetAttackRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<SimpleAttack> {
        const response = await this.getAttackRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Retrieve a tcp port scan\'s results by the attack\'s id
     * Retrieve a tcp port scan\'s results by the attack\'s id
     */
    async getTcpPortScanResultsRaw(requestParameters: GetTcpPortScanResultsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<TcpPortScanResultsPage>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getTcpPortScanResults.');
        }

        if (requestParameters.limit === null || requestParameters.limit === undefined) {
            throw new runtime.RequiredError('limit','Required parameter requestParameters.limit was null or undefined when calling getTcpPortScanResults.');
        }

        if (requestParameters.offset === null || requestParameters.offset === undefined) {
            throw new runtime.RequiredError('offset','Required parameter requestParameters.offset was null or undefined when calling getTcpPortScanResults.');
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
            path: `/api/v1/attacks/{id}/tcpPortScanResults`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => TcpPortScanResultsPageFromJSON(jsonValue));
    }

    /**
     * Retrieve a tcp port scan\'s results by the attack\'s id
     * Retrieve a tcp port scan\'s results by the attack\'s id
     */
    async getTcpPortScanResults(requestParameters: GetTcpPortScanResultsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<TcpPortScanResultsPage> {
        const response = await this.getTcpPortScanResultsRaw(requestParameters, initOverrides);
        return await response.value();
    }
    
    /**
     * Query a certificate transparency log collector.  For further information, see [the explanation](https://certificate.transparency.dev/).  Certificate transparency can be used to find subdomains or related domains.  `retry_interval` is specified in milliseconds.
     * Query a certificate transparency log collector.
     */
    async queryCertificateTransparencyRaw(requestParameters: QueryCertificateTransparencyOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AttackResponse>> {
        if (requestParameters.queryCertificateTransparencyRequest === null || requestParameters.queryCertificateTransparencyRequest === undefined) {
            throw new runtime.RequiredError('queryCertificateTransparencyRequest','Required parameter requestParameters.queryCertificateTransparencyRequest was null or undefined when calling queryCertificateTransparency.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/queryCertificateTransparency`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: QueryCertificateTransparencyRequestToJSON(requestParameters.queryCertificateTransparencyRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AttackResponseFromJSON(jsonValue));
    }

    /**
     * Query a certificate transparency log collector.  For further information, see [the explanation](https://certificate.transparency.dev/).  Certificate transparency can be used to find subdomains or related domains.  `retry_interval` is specified in milliseconds.
     * Query a certificate transparency log collector.
     */
    async queryCertificateTransparency(requestParameters: QueryCertificateTransparencyOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AttackResponse> {
        const response = await this.queryCertificateTransparencyRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Start a tcp port scan  `exclude` accepts a list of ip networks in CIDR notation.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.  Set `max_retries` to 0 if you don\'t want to try a port more than 1 time.
     * Start a tcp port scan
     */
    async scanTcpPortsRaw(requestParameters: ScanTcpPortsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AttackResponse>> {
        if (requestParameters.scanTcpPortsRequest === null || requestParameters.scanTcpPortsRequest === undefined) {
            throw new runtime.RequiredError('scanTcpPortsRequest','Required parameter requestParameters.scanTcpPortsRequest was null or undefined when calling scanTcpPorts.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/scanTcpPorts`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: ScanTcpPortsRequestToJSON(requestParameters.scanTcpPortsRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AttackResponseFromJSON(jsonValue));
    }

    /**
     * Start a tcp port scan  `exclude` accepts a list of ip networks in CIDR notation.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.  Set `max_retries` to 0 if you don\'t want to try a port more than 1 time.
     * Start a tcp port scan
     */
    async scanTcpPorts(requestParameters: ScanTcpPortsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AttackResponse> {
        const response = await this.scanTcpPortsRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
