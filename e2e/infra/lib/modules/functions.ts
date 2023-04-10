import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {Effect, PolicyStatement} from "aws-cdk-lib/aws-iam";

const handler = 'some.handler';

const createRustFunction = (scope: Construct) => (name: string, zip: string, environment: Record<string, string> = {}) =>
    new Function(scope, name, {
        handler,
        code: Code.fromAsset(zip),
        runtime: Runtime.PROVIDED_AL2,
        timeout: Duration.seconds(10),
        memorySize: 512,
        environment,
    })

export const createExampleFunction = (scope: Construct, env: 'dev' | 'prod') => {
    const createRustFunctionForScope = createRustFunction(scope);

    const example = createRustFunctionForScope('TestFunction', '../code/e2e-test-code.zip', {
        ENV: env
    });
    example.addToRolePolicy(new PolicyStatement({
        actions: ['secretsmanager:GetSecretValue'],
        resources: ['*'],
        effect: Effect.ALLOW,
    }))

    return example;
}
