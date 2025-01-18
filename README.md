# Rust WASM pitfalls

## Setup

* `nix develop` - enter the dev shell
* `./build.sh` - build the WASM module (first time)
* `deno task build` - build the WASM module (subsequent times)

## Usage

* `deno run examples/losing_ownership.ts` - run the loss of ownership example

## Licence

This project is licensed under the Blue Oak Model License 1.0.0 - see the [LICENSE.md](LICENSE.md) file for details.
