// Test WebAssembly plugins.

--- plugin-basic ---
#let p = plugin("/assets/plugins/hello.wasm")
#test(p.hello(), bytes("Hello from wasm!!!"))
#test(p.double_it(bytes("hey!")), bytes("hey!.hey!"))
#test(
  p.shuffle(bytes("value1"), bytes("value2"), bytes("value3")),
  bytes("value3-value1-value2"),
)

--- plugin-wrong-number-of-arguments ---
#let p = plugin("/assets/plugins/hello.wasm")

// Error: 2-20 plugin function takes 0 arguments, but 1 was given
#p.hello(bytes(""))

--- plugin-wrong-argument-type ---
#let p = plugin("/assets/plugins/hello.wasm")

// Error: 10-14 expected bytes, found boolean
// Error: 27-29 expected bytes, found integer
#p.hello(true, bytes(()), 10)

--- plugin-error ---
#let p = plugin("/assets/plugins/hello.wasm")

// Error: 2-17 plugin errored with: This is an `Err`
#p.returns_err()

--- plugin-panic ---
#let p = plugin("/assets/plugins/hello.wasm")

// Error: 2-16 plugin panicked: wasm `unreachable` instruction executed
#p.will_panic()
