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
/**
 * 
 * @export
 * @interface ReportingIpPort
 */
export interface ReportingIpPort {
    /**
     * Ip address (v4 or v6)
     * @type {string}
     * @memberof ReportingIpPort
     */
    ip: string;
    /**
     * Port number
     * @type {number}
     * @memberof ReportingIpPort
     */
    port: number;
}

/**
 * Check if a given object implements the ReportingIpPort interface.
 */
export function instanceOfReportingIpPort(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "ip" in value;
    isInstance = isInstance && "port" in value;

    return isInstance;
}

export function ReportingIpPortFromJSON(json: any): ReportingIpPort {
    return ReportingIpPortFromJSONTyped(json, false);
}

export function ReportingIpPortFromJSONTyped(json: any, ignoreDiscriminator: boolean): ReportingIpPort {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'ip': json['ip'],
        'port': json['port'],
    };
}

export function ReportingIpPortToJSON(value?: ReportingIpPort | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'ip': value.ip,
        'port': value.port,
    };
}