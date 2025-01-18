import { consumeFoo, Foo } from "lib";

const foo = new Foo();

// Get bar from foo (works)
const bar1 = foo.bar();
console.log("foo.bar", bar1);

// Consume foo (works)
consumeFoo(foo); // <- transfers ownership of foo to the Rust function
// foo is dropped when the Rust function returns

// Get bar from foo again (fails)
const bar2 = foo.bar(); // <- foo is not valid anymore
console.log("foo.bar", bar2);
