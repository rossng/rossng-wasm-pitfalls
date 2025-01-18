import { consumeFoo, borrowFoo, Foo } from "lib";

const foo = new Foo();

// Borrow foo (works)
borrowFoo(foo);

// Consume foo (works)
consumeFoo(foo); // <- transfers ownership of foo to the Rust function
// foo is dropped when the Rust function returns

// Borrow foo again (fails)
borrowFoo(foo); // <- foo is not valid anymore
