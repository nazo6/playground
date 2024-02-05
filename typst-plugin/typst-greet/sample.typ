#let plugin = plugin("./target/wasm32-unknown-unknown/release/typst_greet.wasm")

#let greet(name) = str(
  plugin.greet(
    bytes(name),
  )
)

#greet("typst")
