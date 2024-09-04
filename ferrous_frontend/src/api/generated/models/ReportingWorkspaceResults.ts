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
import type { ReportingTcpPortScanAttack } from './ReportingTcpPortScanAttack';
import {
    ReportingTcpPortScanAttackFromJSON,
    ReportingTcpPortScanAttackFromJSONTyped,
    ReportingTcpPortScanAttackToJSON,
} from './ReportingTcpPortScanAttack';
import type { ReportingUser } from './ReportingUser';
import {
    ReportingUserFromJSON,
    ReportingUserFromJSONTyped,
    ReportingUserToJSON,
} from './ReportingUser';

/**
 * 
 * @export
 * @interface ReportingWorkspaceResults
 */
export interface ReportingWorkspaceResults {
    /**
     * List of all tcp port scan attacks
     * @type {Array<ReportingTcpPortScanAttack>}
     * @memberof ReportingWorkspaceResults
     */
    tcpPortScanAttacks: Array<ReportingTcpPortScanAttack>;
    /**
     * List of user which started attacks in this workspace
     * @type {Array<ReportingUser>}
     * @memberof ReportingWorkspaceResults
     */
    attacker: Array<ReportingUser>;
}

/**
 * Check if a given object implements the ReportingWorkspaceResults interface.
 */
export function instanceOfReportingWorkspaceResults(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "tcpPortScanAttacks" in value;
    isInstance = isInstance && "attacker" in value;

    return isInstance;
}

export function ReportingWorkspaceResultsFromJSON(json: any): ReportingWorkspaceResults {
    return ReportingWorkspaceResultsFromJSONTyped(json, false);
}

export function ReportingWorkspaceResultsFromJSONTyped(json: any, ignoreDiscriminator: boolean): ReportingWorkspaceResults {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'tcpPortScanAttacks': ((json['tcp_port_scan_attacks'] as Array<any>).map(ReportingTcpPortScanAttackFromJSON)),
        'attacker': ((json['attacker'] as Array<any>).map(ReportingUserFromJSON)),
    };
}

export function ReportingWorkspaceResultsToJSON(value?: ReportingWorkspaceResults | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'tcp_port_scan_attacks': ((value.tcpPortScanAttacks as Array<any>).map(ReportingTcpPortScanAttackToJSON)),
        'attacker': ((value.attacker as Array<any>).map(ReportingUserToJSON)),
    };
}
