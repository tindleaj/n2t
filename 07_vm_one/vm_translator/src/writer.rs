use crate::parser::{MemoryCommand, MemorySegment};
use std::fs::File;

pub struct Writer {
  output: File,
}

impl Writer {
  pub fn new() {
    unimplemented!()
  }

  fn write_arithmetic() {
    unimplemented!()
  }

  fn write_push_pop(command: MemoryCommand, segment: MemorySegment, index: usize) {
    unimplemented!()
  }

  fn close() {
    unimplemented!()
  }
}
