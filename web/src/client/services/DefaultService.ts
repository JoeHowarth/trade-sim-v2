/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AgentInfo } from '../models/AgentInfo';
import type { NetworkShape } from '../models/NetworkShape';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class DefaultService {

    /**
     * Network Shape
     * @returns NetworkShape Successful Response
     * @throws ApiError
     */
    public static networkShape(): CancelablePromise<NetworkShape> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/network/shape',
        });
    }

    /**
     * Price
     * @param tick
     * @returns number Successful Response
     * @throws ApiError
     */
    public static price(
        tick: number,
    ): CancelablePromise<Record<string, number>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/network/{tick}/price',
            path: {
                'tick': tick,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

    /**
     * Market Col
     * @param tick
     * @param field
     * @returns number Successful Response
     * @throws ApiError
     */
    public static marketCol(
        tick: number,
        field: string,
    ): CancelablePromise<Record<string, number>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/network/{tick}/market/{field}',
            path: {
                'tick': tick,
                'field': field,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

    /**
     * List Map Mode
     * @returns string Successful Response
     * @throws ApiError
     */
    public static listMapMode(): CancelablePromise<Array<string>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/network/mapmode',
        });
    }

    /**
     * Init
     * @param scenarioName
     * @returns any Successful Response
     * @throws ApiError
     */
    public static init(
        scenarioName: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/init/{scenario_name}',
            path: {
                'scenario_name': scenarioName,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

    /**
     * Get Agents Pos
     * @param tick
     * @returns AgentInfo Successful Response
     * @throws ApiError
     */
    public static getAgentsPos(
        tick: number,
    ): CancelablePromise<Record<string, AgentInfo>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/agents/{tick}',
            path: {
                'tick': tick,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

}
