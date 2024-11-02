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
  BruteforceSubdomainsRequest,
  HostsAliveRequest,
  QueryCertificateTransparencyRequest,
  QueryDehashedRequest,
  ScanTcpPortsRequest,
  ServiceDetectionRequest,
  SimpleAttack,
  TcpPortScanResultsPage,
  UuidResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    BruteforceSubdomainsRequestFromJSON,
    BruteforceSubdomainsRequestToJSON,
    HostsAliveRequestFromJSON,
    HostsAliveRequestToJSON,
    QueryCertificateTransparencyRequestFromJSON,
    QueryCertificateTransparencyRequestToJSON,
    QueryDehashedRequestFromJSON,
    QueryDehashedRequestToJSON,
    ScanTcpPortsRequestFromJSON,
    ScanTcpPortsRequestToJSON,
    ServiceDetectionRequestFromJSON,
    ServiceDetectionRequestToJSON,
    SimpleAttackFromJSON,
    SimpleAttackToJSON,
    TcpPortScanResultsPageFromJSON,
    TcpPortScanResultsPageToJSON,
    UuidResponseFromJSON,
    UuidResponseToJSON,
} from '../models';

export interface BruteforceSubdomainsOperationRequest {
    bruteforceSubdomainsRequest: BruteforceSubdomainsRequest;
}

export interface DeleteAttackRequest {
    uuid: string;
}

export interface GetAttackRequest {
    uuid: string;
}

export interface GetTcpPortScanResultsRequest {
    uuid: string;
    limit: number;
    offset: number;
}

export interface HostsAliveCheckRequest {
    hostsAliveRequest: HostsAliveRequest;
}

export interface QueryCertificateTransparencyOperationRequest {
    queryCertificateTransparencyRequest: QueryCertificateTransparencyRequest;
}

export interface QueryDehashedOperationRequest {
    queryDehashedRequest: QueryDehashedRequest;
}

export interface ScanTcpPortsOperationRequest {
    scanTcpPortsRequest: ScanTcpPortsRequest;
}

export interface ServiceDetectionOperationRequest {
    serviceDetectionRequest: ServiceDetectionRequest;
}

/**
 * 
 */
export class AttacksApi extends runtime.BaseAPI {

    /**
     * Bruteforce subdomains through a DNS wordlist attack  Enumerate possible subdomains by querying a DNS server with constructed domains. See [OWASP](https://owasp.org/www-community/attacks/Brute_force_attack) for further information.
     * Bruteforce subdomains through a DNS wordlist attack
     */
    async bruteforceSubdomainsRaw(requestParameters: BruteforceSubdomainsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
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

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Bruteforce subdomains through a DNS wordlist attack  Enumerate possible subdomains by querying a DNS server with constructed domains. See [OWASP](https://owasp.org/www-community/attacks/Brute_force_attack) for further information.
     * Bruteforce subdomains through a DNS wordlist attack
     */
    async bruteforceSubdomains(requestParameters: BruteforceSubdomainsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.bruteforceSubdomainsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete an attack and its results
     * Delete an attack and its results
     */
    async deleteAttackRaw(requestParameters: DeleteAttackRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling deleteAttack.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/attacks/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
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
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAttack.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/attacks/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
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
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getTcpPortScanResults.');
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
            path: `/api/v1/attacks/{uuid}/tcpPortScanResults`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
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
     * Check if hosts are reachable  Just an ICMP scan for now to see which targets respond.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.
     * Check if hosts are reachable
     */
    async hostsAliveCheckRaw(requestParameters: HostsAliveCheckRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.hostsAliveRequest === null || requestParameters.hostsAliveRequest === undefined) {
            throw new runtime.RequiredError('hostsAliveRequest','Required parameter requestParameters.hostsAliveRequest was null or undefined when calling hostsAliveCheck.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/hostsAlive`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: HostsAliveRequestToJSON(requestParameters.hostsAliveRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Check if hosts are reachable  Just an ICMP scan for now to see which targets respond.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.
     * Check if hosts are reachable
     */
    async hostsAliveCheck(requestParameters: HostsAliveCheckRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.hostsAliveCheckRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Query a certificate transparency log collector.  For further information, see [the explanation](https://certificate.transparency.dev/).  Certificate transparency can be used to find subdomains or related domains.  `retry_interval` is specified in milliseconds.
     * Query a certificate transparency log collector.
     */
    async queryCertificateTransparencyRaw(requestParameters: QueryCertificateTransparencyOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
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

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Query a certificate transparency log collector.  For further information, see [the explanation](https://certificate.transparency.dev/).  Certificate transparency can be used to find subdomains or related domains.  `retry_interval` is specified in milliseconds.
     * Query a certificate transparency log collector.
     */
    async queryCertificateTransparency(requestParameters: QueryCertificateTransparencyOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.queryCertificateTransparencyRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Query the [dehashed](https://dehashed.com/) API. It provides email, password, credit cards and other types of information from leak-databases.  Note that you are only able to query the API if you have bought access and have a running subscription saved in ferrous.
     * Query the [dehashed](https://dehashed.com/) API.
     */
    async queryDehashedRaw(requestParameters: QueryDehashedOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.queryDehashedRequest === null || requestParameters.queryDehashedRequest === undefined) {
            throw new runtime.RequiredError('queryDehashedRequest','Required parameter requestParameters.queryDehashedRequest was null or undefined when calling queryDehashed.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/queryDehashed`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: QueryDehashedRequestToJSON(requestParameters.queryDehashedRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Query the [dehashed](https://dehashed.com/) API. It provides email, password, credit cards and other types of information from leak-databases.  Note that you are only able to query the API if you have bought access and have a running subscription saved in ferrous.
     * Query the [dehashed](https://dehashed.com/) API.
     */
    async queryDehashed(requestParameters: QueryDehashedOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.queryDehashedRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Start a tcp port scan  `exclude` accepts a list of ip networks in CIDR notation.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.  Set `max_retries` to 0 if you don\'t want to try a port more than 1 time.
     * Start a tcp port scan
     */
    async scanTcpPortsRaw(requestParameters: ScanTcpPortsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
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

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Start a tcp port scan  `exclude` accepts a list of ip networks in CIDR notation.  All intervals are interpreted in milliseconds. E.g. a `timeout` of 3000 means 3 seconds.  Set `max_retries` to 0 if you don\'t want to try a port more than 1 time.
     * Start a tcp port scan
     */
    async scanTcpPorts(requestParameters: ScanTcpPortsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.scanTcpPortsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Perform service detection on a ip and port combination
     * Perform service detection on a ip and port combination
     */
    async serviceDetectionRaw(requestParameters: ServiceDetectionOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.serviceDetectionRequest === null || requestParameters.serviceDetectionRequest === undefined) {
            throw new runtime.RequiredError('serviceDetectionRequest','Required parameter requestParameters.serviceDetectionRequest was null or undefined when calling serviceDetection.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/attacks/serviceDetection`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: ServiceDetectionRequestToJSON(requestParameters.serviceDetectionRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Perform service detection on a ip and port combination
     * Perform service detection on a ip and port combination
     */
    async serviceDetection(requestParameters: ServiceDetectionOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.serviceDetectionRaw(requestParameters, initOverrides);
        return await response.value();
    }

}