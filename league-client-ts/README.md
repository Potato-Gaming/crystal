# Rust API client for openapi

This crate holds all the schemas declared in the OpenAPI schema that is distributed in the League
Client.

## How to keep up to date

The [Rift Explorer](https://github.com/Pupix/rift-explorer) is needed to get the JSON file.

1.  Open Rift Explorer so it restarts League Client in a way that it publishes the Swagger file.

1.  Download the JSON file that is in `https://127.0.0.1:<PORT>/swagger/v2/swagger.json`. Do not
    use `v3/openapi.json` as codegen doesn't seem to be giving good results.

1.  Use the [`openapi-generator`](https://github.com/openapitools/openapi-generator) to generate the
    models. Choose an arbitrary location, like desktop

    ```bash
    openapi-generator generate -i  ./openapi.json -g typescript-axios -o ./some-location --skip-validate-spec
    ```

1.  Remove all unnecessary code. For this package, we only care about type definitions. As all the
    logic will be living in Rust.
