# Rust Lambda in CDK

This repository is a small example how to create a AWS Lambda in Rust and deploy it via CDK.

## Getting Started

Here are step-by-step instructions how I deployed this:
* Checkout this repository
* run `cargo build --release --target x86_64-unknown-linux-musl`
 * If you have problems building the release for the `musl` target:
   * I have made a [github workflow](.github/workflows/rust.yml) to help me building the target (feel free to copy it!)
* `cp ./target/x86_64-unknown-linux-musl/release/bootstrap artifacts/`
* put the `bootstrap` file to the `artifacts` directory
* `cd lambda` -> change into cdk
* run `npm ci` -> to install deps
* run `AWS_PROFILE=YOUR_PROFILE cdk deploy`
 

## Sources

I have read many articles that describe how to create a lambda with rust. 

* [Running Rust Lambda: The Ultimate Guide 2022](https://hevodata.com/learn/rust-lambda/)
* [Writing and deploying Rust Lambda function to AWS: Image glitch as a service](https://www.kirillvasiltsov.com/writing/write-deploy-lambda-image-glitch/)



Special thanks ton those authors! :)
