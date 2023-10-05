/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AgentInfo } from '../models/AgentInfo';
import type { NetworkShape } from '../models/NetworkShape';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class DataService {

    /**
     * Network Shape
     * @param replayName
     * @returns NetworkShape Successful Response
     * @throws ApiError
     */
    public static datanetworkShape(
        replayName: string,
    ): CancelablePromise<NetworkShape> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/data/{replay_name}/network/shape',
            path: {
                'replay_name': replayName,
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
     * @param replayName
     * @returns number Successful Response
     * @throws ApiError
     */
    public static datamarketCol(
        tick: number,
        field: string,
        replayName: string,
    ): CancelablePromise<Record<string, number>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/data/{replay_name}/network/{tick}/market/{field}',
            path: {
                'tick': tick,
                'field': field,
                'replay_name': replayName,
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
    public static datalistMapMode(): CancelablePromise<Array<string>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/data/{replay_name}/network/mapmode',
        });
    }

    /**
     * Get Agents Pos
     * @param tick
     * @param replayName
     * @returns AgentInfo Successful Response
     * @throws ApiError
     */
    public static datagetAgentsPos(
        tick: number,
        replayName: string,
    ): CancelablePromise<Record<string, AgentInfo>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/data/{replay_name}/agents/{tick}',
            path: {
                'tick': tick,
                'replay_name': replayName,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

}
