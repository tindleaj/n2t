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

struct SymbolTable<'a> {
  table: HashMap<&'a str, &'a str>,
}

impl<'a> SymbolTable<'a> {
  pub fn new() -> SymbolTable<'a> {
    let table = SYMBOLS.iter().cloned().collect();

    SymbolTable { table }
  }

  pub fn add_entry() {
    unimplemented!()
  }

  pub fn contains(&self, key: &str) -> bool {
    match &self.table.get(key) {
      Some(val) => return true,
      None => return false,
    }
  }

  pub fn get_addr() {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creates_new_table() {
    let t = SymbolTable::new();
    assert_eq!(t.table, SYMBOLS.iter().cloned().collect())
  }

  fn gets_value_for_existing_key() {
    let t = SymbolTable::new();
    assert_eq!(t.contains("R1"), true);
    assert_eq!(t.contains("Fooey"), false);
  }
}
