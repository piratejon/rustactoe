
pub struct TttSquare {
  pub value : char,
}

impl std::default::Default for TttSquare {
  fn default() -> TttSquare {
    TttSquare {
      value : 'x',
    }
  }
}

impl TttSquare {
  pub fn set_x(&mut self) { self.value = 'x'; }
  pub fn get_value(&self) -> char { 'x' }
}

