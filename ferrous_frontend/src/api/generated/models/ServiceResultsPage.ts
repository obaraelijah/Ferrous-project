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
import type { SimpleService } from './SimpleService';
import {
    SimpleServiceFromJSON,
    SimpleServiceFromJSONTyped,
    SimpleServiceToJSON,
} from './SimpleService';

/**
 * Response containing paginated data
 * @export
 * @interface ServiceResultsPage
 */
export interface ServiceResultsPage {
    /**
     * The page's items
     * @type {Array<SimpleService>}
     * @memberof ServiceResultsPage
     */
    items: Array<SimpleService>;
    /**
     * The limit this page was retrieved with
     * @type {number}
     * @memberof ServiceResultsPage
     */
    limit: number;
    /**
     * The offset this page was retrieved with
     * @type {number}
     * @memberof ServiceResultsPage
     */
    offset: number;
    /**
     * The total number of items this page is a subset of
     * @type {number}
     * @memberof ServiceResultsPage
     */
    total: number;
}

/**
 * Check if a given object implements the ServiceResultsPage interface.
 */
export function instanceOfServiceResultsPage(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "items" in value;
    isInstance = isInstance && "limit" in value;
    isInstance = isInstance && "offset" in value;
    isInstance = isInstance && "total" in value;

    return isInstance;
}

export function ServiceResultsPageFromJSON(json: any): ServiceResultsPage {
    return ServiceResultsPageFromJSONTyped(json, false);
}

export function ServiceResultsPageFromJSONTyped(json: any, ignoreDiscriminator: boolean): ServiceResultsPage {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'items': ((json['items'] as Array<any>).map(SimpleServiceFromJSON)),
        'limit': json['limit'],
        'offset': json['offset'],
        'total': json['total'],
    };
}

export function ServiceResultsPageToJSON(value?: ServiceResultsPage | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'items': ((value.items as Array<any>).map(SimpleServiceToJSON)),
        'limit': value.limit,
        'offset': value.offset,
        'total': value.total,
    };
}
