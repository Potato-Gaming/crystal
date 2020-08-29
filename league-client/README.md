# League Client

This crate holds all the schemas declared in the OpenAPI schema that is distributed in the League
Client.

## Keep up to date

The [Rift Explorer](https://github.com/Pupix/rift-explorer) is needed to get the JSON file.

1.  Open Rift Explorer so it restarts League Client in a way that it publishes the Swagger file.

1.  Download the JSON file that is in `https://127.0.0.1:<PORT>/swagger/v3/openapi.json`.

1.  Use the [`openapi-generator`](https://github.com/openapitools/openapi-generator) to generate the
    models. Choose an arbitrary location, like desktop

    ```bash
    openapi-generator generate -i  ./openapi.json -g rust -o ./some-location --skip-validate-spec
    ```

1.  The previous command generates the `src/models` and `docs/`. But a new `mod.rs` will be needed
    to be in `src/models`. So it can be used for a different crate. To do it easily just do `ls`
    inside `src/models` and use regular expressions to generate the new file.

    In VSCode for example, something like this.

    ```
    # Search for this text
    .+[0-9]+\s([a-z0-9_]+.rs)

    # Then replace it with this
    mod $1;
    pub use self::$1::*;

    # And finally remove all the following
    \.rs
    ```

All these steps are a bit troublesome, but at this moment there's no need for more. Some fancy code
generation may be a good idea t some point.
