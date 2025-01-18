# Rust WASM pitfalls

## Setup

* `nix develop` - enter the dev shell
* `./build.sh` - build the WASM module (first time)
* `deno task build` - build the WASM module (subsequent times)

## Examples

### [Accidental loss of ownership](examples/losing_ownership.ts)

Run this example: `deno run examples/losing_ownership.ts`

This example demonstrates how you can create a Rust object in JavaScript, pass it to a Rust function and unwittingly lose ownership of it.

In most cases, the object will be dropped when the Rust function returns, invalidating the object. When you attempt to use the object again, an exception will be thrown.

### [Holding mutable references across an async yield](examples/mut_async.ts)

Run this example: `deno run examples/mut_async.ts`

This example demonstrates how you can accidentally create a mutable and immutable reference to the same object at the same time.

If you write an async Rust function that takes a mutable reference to an object, the function may yield part-way through execution. The program may then call another Rust function that takes a reference to the same object.

Now there is a mutable reference and an immutable reference to the same object at the same time. This is disallowed and the wasm-bindgen runtime throws an exception.

## Licence

This project is licensed under the Blue Oak Model License 1.0.0 - see the [LICENSE.md](LICENSE.md) file for details.
