# wasmio demo for wasi-cloud

Checkout the companion demo from @danbugs is at [danbugs/wasmio-demo](https://github.com/danbugs/wasmio-demo).

## What if a component can be run portably across clouds and services?

Let's find out!

*Caveat Emptor:* The tooling around building components and the APIs for wasi-cloud are under active development.
Expect many aspects of this to change and note we needed to add a few workarounds.

### Step 1: It starts with a component

We have a component called `pingpong`. This component is built with the latest and greatest
Bytecode Alliance source and is essentially a pure component without any host/framework special sauce.

`pingpong` uses `wasi-messaging` and `wasi-keyvalue` to keep track of the times it was pinged and responds
to pings with pongs and the current ping count.

First we build a preview 1 WASI module, then we migrate this module to preview 2. Note that this
adaption step is temporary until 

```bash
cd pingpong
cargo build --target wasm32-wasi

# now migrate from WASI preview1 to preview2 (to a wasm component)
wasm-tools component new --output ../deploy/components/pingpong.component.wasm --adapt wasi_snapshot_preview1.wasm pingpong.wasm
```

## Step 2: Deploy to a host

Let's imagine for a moment that we just pushed the `pingpong` component to a registry and then pulled it down. The output of the wasm-tools command sent the final `.wasm` to `deploy/components` to simulate this.

There a few other components that I've already built in the components folder.
Some of these could be generated dynamically by the host
or built by a pre-processing step and cached.
This burden is on the platform and not on the author of the component.
If you're a dev targeting components, this next step of host
adaption will be opaque to you as a user (Yay!).

If you're a wasm runtime provider like wasmCloud, well then the next trick
is AWESOME because you can easily adapt changes to WASI APIs as well as
iterate independently at the host layer for any WASI builtins.

```bash
cd ../deploy

# Fuse pingpong with wasmfills to run on wasmCloud
wasm-tools compose --search-path ./components --output ./components/fused.wasm ./components/pingpong.component.wasm

# There should be warnings about WASI things not being found. That's OK!
# The host will provide these imports at link time.

# Next we need to sign the component
# The signature with claims for capabilities are added to a custom section in the wasm binary
wash claims sign --cap "wasmcloud:keyvalue" --cap "wasmcloud:wasi:messaging" --name pingpong ./components/fused.wasm

# This is the fun part. Launch it!
cosmo launch --launch-only
```

## Step 3: Infra things

If you'd like to replicate this demo to it's fullest, then `cosmo up` a wasmCloud host
in all the places and architectures, e.g. in a different cloud like AWS, GCP, or on a Rasberry PI.

In wasmCloud, we can make runtime changes to use different capabilities including changing the
service, e.g. by changing a link, an app can change from running with Redis to Vault.

First we need to start our providers.

```bash
# start redis
redis-cli
```

```bash
# launch a wasi-messaging enabled provider

# todo --js-domain cosmonic ?
wash ctl start provider --host-id <host-id> <provider-ref>

wash ctl link put <actor-id> <provider-id> "wasmcloud:keyvalue" [values]...
wash ctl link put <actor-id> <provider-id> "wasmcloud:wasi:messaging" [values]...
```
