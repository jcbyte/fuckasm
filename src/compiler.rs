use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static BOILERPLATE_ASM: &str = include_str!("main.asm");

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

fn estimate_tape_size(symbols: &Vec<SymbolCount>) -> isize {
    let mut ptr: isize = 0;
    let mut max_ptr = ptr;

    for sym in symbols {
        match sym.symbol {
            Symbol::Right => {
                ptr += sym.count;
                max_ptr = max_ptr.max(ptr);
            }
            Symbol::Left => {
                ptr -= sym.count;
            }
            _ => {}
        }
    }

    return max_ptr + 1;
}

fn write_asm(
    file_path: &Path,
    tape_length: isize,
    symbols: &Vec<SymbolCount>,
) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|_e| "Failed to create file")?;

    let (header, footer) = BOILERPLATE_ASM
        .split_once("<GENERATED_CODE_HERE>")
        .ok_or("Boilerplate code invalid")?;

    write!(
        file,
        "{}",
        header.replace("<CALCULATED_TAPE_LENGTH>", &tape_length.to_string())
    )
    .map_err(|_e| "Failed to write to file")?;

    let mut loop_counter = 0;
    let mut loop_stack: Vec<i32> = Vec::new();

    for sym in symbols {
        let content = match sym.symbol {
            Symbol::Right => format!("add rbx, {}", sym.count),
            Symbol::Left => format!("sub rbx, {}", sym.count),
            Symbol::Incr => format!("add byte [d + rbx], {}", sym.count),
            Symbol::Decr => format!("sub byte [d + rbx], {}", sym.count),
            Symbol::Out => "call print".into(),
            Symbol::In => "call read".into(),
            Symbol::StartLoop => {
                let content = format!(
                    "loop_start_{0}:\ncmp byte [d + rbx], 0\nje loop_end_{0}",
                    loop_counter
                );

                loop_stack.push(loop_counter);
                loop_counter += 1;

                content
            }
            Symbol::EndLoop => {
                let stack_counter = loop_stack.pop().ok_or("Unmatched ']'")?;
                format!("jmp loop_start_{0}\nloop_end_{0}:", stack_counter)
            }
        };

        writeln!(file, "{}", content).map_err(|_e| "Failed to write to file")?;
    }

    write!(file, "{}", footer).map_err(|_e| "Failed to write to file")?;

    return Ok(());
}

pub fn compile(input_file: &Path, output_file: &Path) -> Result<(), String> {
    let code = fs::read_to_string(input_file).map_err(|_e| "Failed to read file")?;

    let symbols: Vec<ReducedSymbolCount> = code
        .chars()
        .filter_map(Symbol::get) // ignores all non BF syntax
        .map(SymbolCount::new)
        .map(|s| s.into())
        .collect();

    let folded_symbols: Vec<SymbolCount> = fold_symbols(symbols)
        .into_iter()
        .map(|s| s.into())
        .collect();

    let tape_length_estimate = estimate_tape_size(&folded_symbols);
    write_asm(output_file, tape_length_estimate, &folded_symbols)?;

    return Ok(());
}

// todo:
// - readme
