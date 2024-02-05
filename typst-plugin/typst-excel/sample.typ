#let plugin = plugin("./target/wasm32-unknown-unknown/release/typst_excel.wasm")

#let get_table(file, sheet, col, row, width, height) = csv.decode(
  plugin.get_table(
    read(file, encoding: none),
    bytes(sheet),
    bytes(col),
    bytes(row),
    bytes(width),
    bytes(height)
  ),
  delimiter: "\t"
)

#table(
  columns: 4,
  ..get_table("Book1.xlsx","Sheet1","1","1","4","10").flatten()
)
