import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {createExampleFunction} from "./modules/functions";

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    createExampleFunction(this, 'dev')
    // TODO also test for prod
  }
}
