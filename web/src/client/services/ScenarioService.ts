/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Scenario_Input } from '../models/Scenario_Input';
import type { Scenario_Output } from '../models/Scenario_Output';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ScenarioService {

    /**
     * All
     * @returns string Successful Response
     * @throws ApiError
     */
    public static scenarioall(): CancelablePromise<Array<string>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/scenario/',
        });
    }

    /**
     * Run Scenario
     * @param name
     * @param requestBody
     * @returns boolean Successful Response
     * @throws ApiError
     */
    public static scenariorunScenario(
        name: string = 'last',
        requestBody?: (Scenario_Input | null),
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/scenario/',
            query: {
                'name': name,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                422: `Validation Error`,
            },
        });
    }

    /**
     * Get
     * @param name
     * @returns Scenario_Output Successful Response
     * @throws ApiError
     */
    public static scenarioget(
        name: string,
    ): CancelablePromise<Scenario_Output> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/scenario/{name}',
            path: {
                'name': name,
            },
            errors: {
                422: `Validation Error`,
            },
        });
    }

    /**
     * Post
     * @param name
     * @param requestBody
     * @returns any Successful Response
     * @throws ApiError
     */
    public static scenariopost(
        name: string,
        requestBody: Scenario_Input,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/scenario/{name}',
            path: {
                'name': name,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                422: `Validation Error`,
            },
        });
    }

}
