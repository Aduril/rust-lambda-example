import * as cdk from "@aws-cdk/core";
import * as lambda from "@aws-cdk/aws-lambda";

export class LambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    new lambda.Function(this, "LambdaHandler", {
      code: lambda.Code.fromAsset("../artifacts"),
      handler: "unrelated",
      runtime: lambda.Runtime.PROVIDED_AL2,
      functionName: "BabysVeryFirstRustLambda",
    });
    // The code that defines your stack goes here
  }
}
