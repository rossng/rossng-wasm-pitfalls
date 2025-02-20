import { useJsVec, useRsVec, useExternVec, Baz } from "lib";

// Example 1: array of native JavaScript values
const arr1 = ["foo", "bar", "baz"];

useJsVec(arr1); // <- the bindings create a new Vec<String> from the JavaScript array
// The Vec<String> is dropped when the Rust function returns,
// but no problem! The JavaScript array and strings are still alive.

useJsVec(arr1); // <- succeeds again

// Example 2: array of Rust objects
const arr2 = [new Baz(), new Baz(), new Baz()];

useRsVec(arr2); // <- transfers ownership of all the Baz instances
// All Baz instances are dropped when the Rust function returns

// On the second use, even though a new Vec is created, the
// Rust function will crash because the Baz instances have been dropped.
try {
  useRsVec(arr2);
} catch (_) {
  console.error("Failed to call `useRsVec(arr2)` a second time");
}

// Example 3: manually clone the Baz instances before passing them to Rust

const arr3 = [new Baz(), new Baz(), new Baz()];

useRsVec(arr3.map((baz) => baz.clone()));
useRsVec(arr3.map((baz) => baz.clone())); // <- succeeds a second time

// Example 4: don't let wasm-bindgen do the conversion from JavaScript array to Rust Vec

const arr4 = [new Baz(), new Baz(), new Baz()];

useExternVec(arr4); // <- all the Baz instances are cloned by the TryInto impl
useExternVec(arr4); // <- succeeds a second time
