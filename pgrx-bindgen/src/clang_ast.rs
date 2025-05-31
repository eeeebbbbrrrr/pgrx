use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

pub type SymbolName = String;
pub type LineNumber = usize;
pub type ColumnNumber = usize;
#[derive(Debug, Eq, PartialEq)]
pub struct HeaderInfo {
    path: PathBuf,
    location: (LineNumber, ColumnNumber),
}

pub struct SymbolLookup {
    target_includes: Vec<String>,
    mapping: HashMap<SymbolName, HeaderInfo>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SymbolLocation<'a> {
    Postgres { header_info: &'a HeaderInfo, mod_path: String },
    Elsewhere { header_info: &'a HeaderInfo },
}

impl SymbolLookup {
    pub fn location(&self, symbol: &str) -> Option<SymbolLocation> {
        let header_info = self.mapping.get(symbol)?;
        let header_path = header_info.path.display().to_string();
        let header_path = header_path.as_str();
        let mut path = header_path;
        for target in &self.target_includes {
            if path.starts_with(target) {
                path = path.trim_start_matches(target);
                break;
            }
        }
        if path == header_path {
            return Some(SymbolLocation::Elsewhere { header_info });
        }

        let mut path = path.rsplitn(2, '.');
        let _ = path.next()?; // drop extension
        let path = path.next()?;
        let mod_path = path.replace('/', "::");
        Some(SymbolLocation::Postgres { header_info, mod_path })
    }
}

pub fn parse_ast(target_includes: Vec<String>, ast: String) -> eyre::Result<SymbolLookup> {
    let mut mapping = HashMap::<SymbolName, HeaderInfo>::new();
    let mut bufreader = BufReader::new(ast.as_bytes());
    loop {
        let block = read_block(&mut bufreader)?;
        if block.is_empty() {
            break;
        }

        let mut spelling = None;
        let mut location = None;
        for line in block {
            if line.starts_with("spelling = ") {
                let mut parts = line.split('"');
                let _ = parts.next(); // everything to the left of the first quote
                spelling = parts.next().map(String::from);
            } else if line.starts_with("location = ") {
                let mut parts = line.split(" = ");
                let _ = parts.next(); // everything to the left of the equal sign
                let path = parts.next().unwrap();

                // these are parsing backwards (right-to-left)
                let mut parts = path.rsplitn(3, ':');
                let Some(Ok(col_number)) = parts.next().map(usize::from_str) else {
                    continue;
                }; // pop column number
                let Some(Ok(line_number)) = parts.next().map(usize::from_str) else {
                    continue;
                }; // pop line number

                let Some(path) = parts.next() else {
                    continue;
                };
                location = Some(HeaderInfo {
                    path: PathBuf::from(path),
                    location: (line_number, col_number),
                })
            } else if spelling.is_some() && location.is_some() {
                break;
            }
        }

        if spelling.is_none() || location.is_none() {
            continue;
        }

        mapping.insert(spelling.unwrap(), location.unwrap());
    }

    Ok(SymbolLookup { target_includes, mapping })
}

fn read_block(bufreader: &mut BufReader<&[u8]>) -> std::io::Result<Vec<String>> {
    let mut block = Vec::new();

    let mut open = 0;
    for line in bufreader.lines() {
        let line = line?;
        let line = line.trim();
        if line == "(" {
            open += 1;
        }
        if line == ")" {
            open -= 1;
            if open == 0 {
                break;
            }
        }

        block.push(line.to_string());
    }

    Ok(block)
}
