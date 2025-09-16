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
    count: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ReducedSymbolCount {
    symbol: Symbol,
    count: isize,
}

impl SymbolCount {
    fn new(sym: Symbol) -> Self {
        SymbolCount {
            symbol: sym,
            count: 1,
        }
    }

    fn reduce(self) -> ReducedSymbolCount {
        return match self.symbol {
            Symbol::Left => ReducedSymbolCount {
                symbol: Symbol::Right,
                count: -self.count,
            },
            Symbol::Decr => ReducedSymbolCount {
                symbol: Symbol::Incr,
                count: -self.count,
            },
            _ => ReducedSymbolCount {
                symbol: self.symbol,
                count: self.count,
            },
        };
    }
}

impl ReducedSymbolCount {
    fn normalise(self) -> SymbolCount {
        if self.count >= 0 {
            return SymbolCount {
                symbol: self.symbol,
                count: self.count,
            };
        }

        let flipped_symbol = match self.symbol {
            Symbol::Left => Symbol::Right,
            Symbol::Right => Symbol::Left,
            Symbol::Incr => Symbol::Decr,
            Symbol::Decr => Symbol::Incr,
            other => other,
        };

        return SymbolCount {
            symbol: flipped_symbol,
            count: self.count.abs(),
        };
    }
}

impl From<SymbolCount> for ReducedSymbolCount {
    fn from(sym: SymbolCount) -> Self {
        return sym.reduce();
    }
}

impl From<ReducedSymbolCount> for SymbolCount {
    fn from(sym: ReducedSymbolCount) -> Self {
        return sym.normalise();
    }
}

fn fold_symbols(reduced_symbols: Vec<ReducedSymbolCount>) -> Vec<ReducedSymbolCount> {
    let mut collapsed: Vec<ReducedSymbolCount> = Vec::new();

    for sym in &reduced_symbols {
        match sym.symbol {
            Symbol::Right | Symbol::Incr => {
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

    let symbols: Vec<ReducedSymbolCount> = code
        .chars()
        .filter_map(Symbol::get)
        .map(SymbolCount::new)
        .map(|s| s.into())
        .collect();

    let folded_symbols: Vec<SymbolCount> = fold_symbols(symbols)
        .into_iter()
        .map(|s| s.into())
        .collect();

    dbg!(folded_symbols);
}
