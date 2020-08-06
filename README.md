# rust-rest-dynamodb

An example REST Rust web service, with a DynamoDB backend.

This project uses the [rocket.rs](rocket.rs) web framework to define an API for creating, reading, updating, deleting and listing products.

It uses [Rusoto](https://github.com/rusoto/rusoto) and [Dynomite](https://github.com/softprops/dynomite) for accessing Amazon DynamoDB.

Note: This repository currently uses the `master` branch of Rocket, due to support for async/tokio.


## Running this example

First install the Rust toolchain, by following the instructions at: https://rustup.rs/

Then clone the repo:

```
$ git clone https://github.com/PaulMaddox/rust-rest-dynamodb.git
$ cd rust-rest-dynamodb
```

You will need to make sure that:

 - You have valid AWS credentials (either env variables, IAM role or ~/.aws/credentials file)
 - You create a DynamoDB table in advance, called `products`. It only needs a single field defined (the partition key) called `id`, of type String.
 - The region in main.rs matches the region you have the DynamoDB table deployed to

... and finally run the project with cargo

```
$ cargo run 
```
