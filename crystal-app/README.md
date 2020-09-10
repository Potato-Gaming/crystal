# crystal-app

Front End for the Crystal Application

## Development

### Requirements

1.  Follow instructions for [Tauri](https://tauri.studio/en/docs/getting-started/intro)

    To make sure all the dependencies needed are ready run

    ```bash
    npm run info
    ```

1.  **For Windows** Install webview2, which uses Edge/Chromium so it's compatible with new JS
    features and a better environment experience.
    [Download here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/),
    More information about webview2 can be found
    [here](https://docs.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution).

### Run in development mode

```bash
npm start
```

To log data from the core library you can do the following

```bash
RUST_LOG=crystal_core=debug npm start
```

For Windows the environment variable is set up like following

```powershell
$env:RUST_LOG="crystal_core=debug"
npm start
```
