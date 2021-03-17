## WasmLib for Rust

This is a clone of IOTA Foundations Wasmlib, made on 14.03.2012. The current commit is bb6563a@master (https://github.com/iotaledger/wasp/commit/bb6563aa415b52ce361a1a776d6358cf6e8cd958). 

**This is a MODIFIED version**. Any changes are **ONLY** focused on improving code readability. No function has been or will be changed in any way.

This repository will be removed when wasmlib is publish on crates.io or is kept in its own repo.

---

`WasmLib` allows developers to use Rust to create smart contracts for ISCP that
compile into Wasm and can run directly on ISCP-enabled Wasp nodes and on the
Solo environment.

`WasmLib` treats the programming of smart contracts as simple access to a
key/value data and token storage where smart contract properties, request
parameters, token balances and the smart contract state can be accessed in a
universal, consistent way.

The _wasmlib_ folder provides the interface to the VM sandbox provided by the
Wasp node through _ScFuncContext_ and _ScViewContext_.