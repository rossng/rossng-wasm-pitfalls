import { Bar, useBar, useMutBar } from "lib";

const bar = new Bar();

// Schedule `useBar` to run when the current task yields
// `useBar` takes an immutable reference to `bar`
setTimeout(() => useBar(bar), 0);

// Start running `useMutBar` immediately
// `useMutBar` takes a mutable reference to `bar`
await useMutBar(bar);

// When `useMutBar` sleeps (and therefore yields), it
// continues to hold a mutable reference to `bar`.
// Then `useBar` runs, and takes an immutable reference to `bar`.
// It is not allowed to have a mutable reference and an immutable reference
// to the same object at the same time, causing a runtime crash.
