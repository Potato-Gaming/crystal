# crystal-app

Front End for the Crystal Application

## Development

### Requirements

1.  Follow instructions for [Tauri](https://tauri.studio/en/docs/getting-started/intro)

1.  Make sure crystal-core is compiled for production otherwise you'll need t

    ```
    cargo build --release --bin=crystal-core
    ```

1.  To run crystal-core when application starts, setup the environment variable: `CRYSTAL_CORE_BIN`
    otherwise you'll need to run the binary in another terminal. Which is useful during development
    process.

To make sure all the dependencies needed are ready run

```bash
npm run info
```

### Run in development mode

```bash
npm start
```

Windows may need to setup the environment first

```powershell
 # This is an optional flag, when set it'll try to run the binary.
 # For development p

 $env:CRYSTAL_CORE_BIN="C:\<PATH>\crystal\target\release\crystal-core.exe"
 npm start
```
