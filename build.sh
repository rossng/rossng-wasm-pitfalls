set -euo pipefail

# run wasm-pack in the lib/ directory
(
cd lib-rs
wasm-pack build --out-dir ../lib-js
)
