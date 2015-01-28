
enum TttSquareValue {
  X, O, Blank
}

pub struct TttSquare {
  value : TttSquareValue,
}

impl std::default::Default for TttSquare {
  fn default() -> TttSquare {
    TttSquare {
      value : TttSquareValue::X,
    }
  }
}

impl TttSquare {
  pub fn set_x(&mut self) { self.value = TttSquareValue::X; }
  pub fn get_value(&self) -> char { 'x' }
}

