use calamine::{Reader, Xlsx, XlsxError};
use wasm_minimal_protocol::*;

initiate_protocol!();

fn parse_num(num: &[u8]) -> Result<usize, String> {
    std::str::from_utf8(num)
        .map_err(|e| format!("Invalid number: {e}"))?
        .parse()
        .map_err(|e| format!("Invalid number: {e}"))
}

#[wasm_func]
pub fn get_table(
    data: &[u8],
    sheet: &[u8],
    col: &[u8],
    row: &[u8],
    width: &[u8],
    height: &[u8],
) -> Result<Vec<u8>, String> {
    let sheet = std::str::from_utf8(sheet).map_err(|e| format!("Invalid sheet name: {e}"))?;
    let col = parse_num(col)?;
    let row = parse_num(row)?;
    let width = parse_num(width)?;
    let height = parse_num(height)?;

    let cursor = std::io::Cursor::new(data);
    let mut workbook: Xlsx<_> = calamine::open_workbook_from_rs(cursor)
        .map_err(|e: XlsxError| format!("Failed to open workbook: {e}"))?;
    let sheet_range = workbook
        .worksheet_range(sheet)
        .map_err(|e| format!("Sheet read error: {e}"))?;

    let mut lines = vec![];

    for r in row..row + height {
        let mut line = vec![];
        for c in col..col + width {
            if let Some(cell) =
                sheet_range.get_value((r.try_into().unwrap(), c.try_into().unwrap()))
            {
                line.push(cell.to_string());
            } else {
                line.push("null".to_string());
            }
        }
        lines.push(line.join("\t"));
    }

    Ok(lines.join("\n").into_bytes())
}
