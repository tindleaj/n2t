use std::collections::HashMap;

const SYMBOLS: [(&str, &str); 22] = [
  ("SP", "0"),
  ("LCL", "1"),
  ("ARG", "2"),
  ("THIS", "3"),
  ("THAT", "4"),
  ("R0", "0"),
  ("R1", "1"),
  ("R2", "2"),
  ("R3", "3"),
  ("R4", "4"),
  ("R5", "5"),
  ("R6", "6"),
  ("R7", "7"),
  ("R8", "8"),
  ("R9", "9"),
  ("R10", "10"),
  ("R11", "11"),
  ("R13", "13"),
  ("R14", "14"),
  ("R15", "15"),
  ("SCREEN", "16384"),
  ("KBD", "24567"),
];

pub struct SymbolTable {
  table: HashMap<String, String>,
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    let table = SYMBOLS
      .iter()
      .map(|pair| (pair.0.to_string(), pair.1.to_string()))
      .collect();

    SymbolTable { table }
  }

  pub fn add_entry(&mut self, symbol: &str, value: &str) {
    self.table.insert(symbol.into(), value.into());
  }

  pub fn contains(&self, key: &str) -> bool {
    match &self.table.get(key) {
      Some(_val) => return true,
      None => return false,
    }
  }

  pub fn get_addr(&self, key: &str) -> &str {
    &self.table.get(key).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creates_new_table() {
    let t = SymbolTable::new();
    assert_eq!(
      t.table,
      SYMBOLS
        .iter()
        .map(|pair| (pair.0.to_string(), pair.1.to_string()))
        .collect()
    );
  }

  #[test]
  fn checks_if_val_exists() {
    let t = SymbolTable::new();
    assert_eq!(t.contains("R1"), true);
    assert_eq!(t.contains("Fooey"), false);
  }

  #[test]
  fn adds_entry() {
    let mut t = SymbolTable::new();
    t.add_entry("test", "value");
    assert_eq!(t.contains("test"), true)
  }

  #[test]
  fn gets_value_at_address() {
    let t = SymbolTable::new();
    assert_eq!(t.get_addr("SP"), "0");
  }
}
