# Bare Metal Local Setup

Each workspace within Varlog is created using Rust, from the backend all the way
to the frontend. To get started, you'll want to make sure you have
[Rust](https://www.rust-lang.org/) installed. As of this writing, Varlog is built
successfully with version 1.69.0. One caveat is that [Trunk](https://trunkrs.dev/)
is used to _Build, bundle & ship_ the frontend Rust WASM application, found in the
`app` workspace. We'll cover that one latter.

## Server and Registry (Backend)

Both the `server` and `registry` workspace can be built using Rust. To do this,
you'll want to clone the repo and run `cargo build`. This will build a development
target and download/compile all of the dependencies across all of the workspaces.
Once this is done, you will only need to download and compile dependencies when
new ones are added.

## App (Frontend)
Varlogs frontend is created using [Yew](https://yew.rs/), _A framework for creating
reliable and efficient web applications_. Since this application needs to be bundled
with WASM, the build system is a _bit_ different. You'll want to `cargo install
--locked trunk` in order to install trunk to your path. At this point, you should be
all set to run the app locally. To do this, simply run `make app`. This will create
a development server which will be updated on each change to the apps source code.

## Putting it all together

Now that all dependencies and build tools have been installed, you're ready to start
the local development Varlog servers! In order for the system to startup properly,
**you need to start the registry first**. The `server` workspace is dependent on the
registry being up and running, and the local development setup will not work without
it.

You can start either the `registry` or the `app` first, but here are the recommended
commands to run (each in their own shell):

1. `make registry` → Runs on port `:8888` _wait until the the workers are running_.
2. `make server` → Runs on port `:8080`
3. `make app` → Runs on port `:8000`

If you see the following outputs:

```
# registry
$ make registry
cargo run -p registry
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/varlog_registry`
[2023-04-27T03:41:34Z INFO  actix_server::builder] starting 10 workers
[2023-04-27T03:41:34Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2023-04-27T03:41:36Z INFO  actix_web::middleware::logger] 127.0.0.1 "POST /register HTTP/1.1" 204 0 "-" "-" 0.000389

 # server
$ make server
cargo run -p server
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/varlog_server`
03:41:36 [WARN] Could not create log file /var/log/varlog.log: Permission denied (os error 13).
03:41:36 [INFO] Successfully registered hostname.
03:41:36 [INFO] starting 10 workers
03:41:36 [INFO] Actix runtime found; starting in Actix runtime# app

# app
$ make app
trunk serve --proxy-backend=http://localhost:8080/v1 --address=0.0.0.0 --port=8000 app/index.h
tml
2023-04-27T03:39:55.272031Z  INFO 📦 starting build
2023-04-27T03:39:55.273230Z  INFO spawning asset pipelines
2023-04-27T03:39:55.525026Z  INFO building app
2023-04-27T03:39:55.526510Z  INFO compiling sass/scss path="app/index.scss"
2023-04-27T03:39:55.728798Z  INFO finished compiling sass/scss path="app/index.scss"
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
2023-04-27T03:39:55.764366Z  INFO fetching cargo artifacts
2023-04-27T03:39:55.866485Z  INFO processing WASM for app
2023-04-27T03:39:55.884411Z  INFO using system installed binary app=wasm-bindgen version=0.2.84
2023-04-27T03:39:55.884509Z  INFO calling wasm-bindgen for app
2023-04-27T03:39:55.979421Z  INFO copying generated wasm-bindgen artifacts
2023-04-27T03:39:55.981091Z  INFO applying new distribution
2023-04-27T03:39:55.981950Z  INFO ✅ success
2023-04-27T03:39:55.983413Z  INFO 📡 serving static assets at -> /
2023-04-27T03:39:55.983419Z  INFO 📡 proxying /v1 -> http://localhost:8080/v1
2023-04-27T03:39:55.983488Z  INFO 📡 server listening at http://0.0.0.0:8000
```

Then congratulations 🎉 You can now visit [localhost:8000](http://localhost:8000)
and interact with the UI.


### Troubleshooting

If you visit the UI and don't see any servers, check the `server`'s logs. If you
see a log like the following, you need to restart the server after the registry
has been started.
```
[WARN] Error while registering hostname: error sending request for url (http://localhost:8888/register): error trying to connect: tcp connect error: Connection refused (os error 61).
```

