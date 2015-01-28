
pub struct TttSquare {
  pub value : char,
}

impl TttSquare {
  pub fn set_x(&mut self) { self.value = 'x'; }
  pub fn get_value(&self) -> char { return 'x'; }
}

