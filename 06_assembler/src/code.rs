const DEST_TABLE: [(&str, &str); 8] = [
  ("null", "000"),
  ("M", "001"),
  ("D", "010"),
  ("MD", "011"),
  ("A", "100"),
  ("AM", "101"),
  ("AD", "110"),
  ("AMD", "111"),
];

const JUMP_TABLE: [(&str, &str); 8] = [
  ("null", "000"),
  ("JGT", "001"),
  ("JEQ", "010"),
  ("JGE", "011"),
  ("JLT", "100"),
  ("JNE", "101"),
  ("JLE", "110"),
  ("JMP", "111"),
];

const COMP_TABLE: [(&str, &str); 28] = [
  ("0", "0101010"),
  ("1", "0111111"),
  ("-1", "0111010"),
  ("D", "0001100"),
  ("A", "0110000"),
  ("!D", "0001101"),
  ("!A", "0110001"),
  ("-D", "0001111"),
  ("-A", "0110011"),
  ("D+1", "0011111"),
  ("A+1", "0110111"),
  ("D-1", "0001110"),
  ("A-1", "0110010"),
  ("D+A", "0000010"),
  ("D-A", "0010011"),
  ("A-D", "0000111"),
  ("D&A", "0000000"),
  ("D|A", "0010101"),
  ("M", "1110000"),
  ("!M", "1110001"),
  ("-M", "1110011"),
  ("M+1", "1110111"),
  ("M-1", "1110010"),
  ("D+M", "1000010"),
  ("D-M", "1010011"),
  ("M-D", "1000111"),
  ("D&M", "1000000"),
  ("D|M", "1010101"),
];

pub fn dest(key: &str) -> Option<&str> {
  for pair in DEST_TABLE.iter() {
    if pair.0 == key {
      return Some(pair.1);
    }
  }
  None
}

pub fn jump(key: &str) -> Option<&str> {
  for pair in JUMP_TABLE.iter() {
    if pair.0 == key {
      return Some(pair.1);
    }
  }
  None
}

pub fn comp(key: &str) -> Option<&str> {
  for pair in COMP_TABLE.iter() {
    if pair.0 == key {
      return Some(pair.1);
    }
  }
  None
}

pub fn contains_comp(line: &str) -> bool {
  for pair in COMP_TABLE.iter() {
    if line.contains(pair.0) {
      return true;
    }
  }

  false
}
