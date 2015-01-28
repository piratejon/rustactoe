
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
  pub fn set_x(&mut self) { self.value = TttSquareValue::X }
  pub fn set_o(&mut self) { self.value = TttSquareValue::O }
  pub fn set_blank(&mut self) { self.value = TttSquareValue::Blank }
  pub fn get_value(&self) -> char {
    match self.value {
      TttSquareValue::X => 'x',
      TttSquareValue::O => 'o',
      TttSquareValue::Blank => '_',
    }
  }
}

