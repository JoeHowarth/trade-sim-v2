/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ReplayInfo } from '../models/ReplayInfo';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ReplayService {

    /**
     * All
     * @returns any Successful Response
     * @throws ApiError
     */
    public static replayall(): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/replay/',
        });
    }

    /**
     * Get Info
     * @param name
     * @returns ReplayInfo Successful Response
     * @throws ApiError
     */
    public static replaygetInfo(
        name: string,
    ): CancelablePromise<ReplayInfo> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/replay/info/{name}',
            path: {
                'name': name,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

}
