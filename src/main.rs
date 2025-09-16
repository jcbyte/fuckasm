use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Symbol {
    Right,
    Left,
    Incr,
    Decr,
    Out,
    In,
    StartLoop,
    EndLoop,
}

impl Symbol {
    fn as_char(self) -> char {
        match self {
            Symbol::Right => '>',
            Symbol::Left => '<',
            Symbol::Incr => '+',
            Symbol::Decr => '-',
            Symbol::Out => '.',
            Symbol::In => ',',
            Symbol::StartLoop => '[',
            Symbol::EndLoop => ']',
        }
    }

    fn get(c: char) -> Option<Self> {
        match c {
            '>' => Some(Symbol::Right),
            '<' => Some(Symbol::Left),
            '+' => Some(Symbol::Incr),
            '-' => Some(Symbol::Decr),
            '.' => Some(Symbol::Out),
            ',' => Some(Symbol::In),
            '[' => Some(Symbol::StartLoop),
            ']' => Some(Symbol::EndLoop),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SymbolCount {
    symbol: Symbol,
    count: usize,
}

fn collapse_symbols(symbols: Vec<SymbolCount>) -> Vec<SymbolCount> {
    let mut collapsed: Vec<SymbolCount> = Vec::new();

    for sym in &symbols {
        match sym.symbol {
            Symbol::Right | Symbol::Left | Symbol::Incr | Symbol::Decr => {
                if let Some(last) = collapsed.last_mut() {
                    // Collapse into previous if same type
                    if last.symbol == sym.symbol {
                        last.count += sym.count;
                        continue;
                    }
                }

                collapsed.push(sym.clone());
            }
            _ => collapsed.push(sym.clone()),
        }
    }

    return collapsed;
}

fn main() {
    let filename = "main.bf";

    let code = fs::read_to_string(filename).expect("Failed to read file");

    let symbols: Vec<SymbolCount> = code
        .chars()
        .filter_map(Symbol::get)
        .map(|s| SymbolCount {
            symbol: s,
            count: 1,
        })
        .collect();

    let c = collapse_symbols(symbols);
    dbg!(c);
}
