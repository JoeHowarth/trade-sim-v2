/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
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

}
