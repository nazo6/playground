#let plugin = plugin("./target/wasm32-unknown-unknown/release/typst_minimal_plugin.wasm")

#let greet(name) = str(
  plugin.greet(
    bytes(name),
  )
)

#greet("user")
