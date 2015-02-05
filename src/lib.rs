
use std::str;

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
  pub fn set_value(&mut self, value : char) {
    match value {
      'X' | 'x' => self.set_x(),
      'O' | 'o' => self.set_o(),
      _ => self.set_blank(),
    }
  }
}

pub struct TttBoard {
  squares : [TttSquare; 9],
}

impl std::default::Default for TttBoard {
  fn default() -> TttBoard {
    TttBoard {
      squares : [
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
      ]
    }
  }
}

impl TttBoard {
  pub fn count_blanks(&self) -> usize {
    let mut count = 0;
    for i in self.squares.iter() {
      if i.get_value() == '_' {
        count += 1;
      }
    }
    count
  }

  pub fn get_square(&self, index : usize) -> char {
    /*
       1 | 2 | 3
       ---------
       4 | 5 | 6
       ---------
       7 | 8 | 9
       */
    self.squares[index-1].get_value()
  }

  pub fn set_square(&mut self, index : usize, value : char) {
    self.squares[index-1].set_value(value)
  }

  pub fn winner(&self) -> char {
    if self.get_square(7) == 'x' && self.get_square(8) == 'x' && self.get_square(9) == 'x' {
      'x'
    } else if self.get_square(1) == 'o' && self.get_square(5) == 'o' && self.get_square(9) == 'o' {
      'o'
    } else if self.get_square(1) == 'x' && self.get_square(5) == 'x' && self.get_square(9) == 'x' {
      'x'
    } else {
      '_'
    }
  }

  pub fn reset(&mut self) {
    for i in range(0, self.squares.len()) {
      self.squares[i].set_value('_')
    }
  }
}

pub struct TttBoardBinRep {
  xs : u16,
  os : u16,
}

impl std::default::Default for TttBoardBinRep {
  fn default() -> TttBoardBinRep {
    TttBoardBinRep {
      xs : 0,
      os : 0,
    }
  }
}

impl TttBoardBinRep {
  pub fn count_blanks(&self) -> u16 {
    let mut count = 9;
    for i in range(0,9) {
      count -= (((self.xs | self.os) >> i) & 1)
    }
    count
  }

  pub fn get_square(&self, i : u8) -> char {
    if self.xs & (1 << (i-1)) > 0 { 'x' }
    else if self.os & (1 << (i-1)) > 0 { 'o' }
    else { '_' }
  }

  pub fn set_square(&mut self, i : u8, value : char) {
    if value == 'x' || value == 'X' {
      self.set_x(i)
    } else if value == 'o' || value == 'O' {
      self.set_o(i)
    } else {
      self.set_blank(i)
    }
  }

  pub fn disable_x(&mut self, i : u8) {
    self.xs &= (0b111_111_111 ^ (1 << (i-1)));
  }

  pub fn disable_o(&mut self, i : u8) {
    self.os &= (0b111_111_111 ^ (1 << (i-1)));
  }

  pub fn set_x(&mut self, i : u8) {
    self.disable_o(i);
    self.xs |= (1 << (i-1))
  }

  pub fn set_o(&mut self, i : u8) {
    self.disable_x(i);
    self.os |= (1 << (i-1))
  }

  pub fn set_blank(&mut self, i : u8) {
    self.disable_x(i);
    self.disable_o(i);
  }

  pub fn reset(&mut self) {
    self.xs = 0;
    self.os = 0;
  }

  pub fn check_positions(&self, p : u16) -> bool {
    p & 0b111_000_000 == 0b111_000_000
      || p & 0b000_111_000 == 0b000_111_000
      || p & 0b000_000_111 == 0b000_000_111
      || p & 0b100_100_100 == 0b100_100_100
      || p & 0b010_010_010 == 0b010_010_010
      || p & 0b001_001_001 == 0b001_001_001
      || p & 0b100_010_001 == 0b100_010_001
      || p & 0b001_010_100 == 0b001_010_100
  }

  pub fn winner(&mut self) -> char {
    if self.check_positions(self.xs) { 'x' }
    else if self.check_positions(self.os) { 'o' }
    else { '_' }
  }

  fn who_has_it(&self, i : u8) -> char {
    if self.xs & (1 << i) > 0 {
      'x'
    } else if self.os & (1 << i) > 0 {
      'o'
    } else {
      '_'
    }
  }

  pub fn from_string(&mut self, state : &str) {
    if (state.len() == 9) {
      self.set_square(1, state.char_at(0));
      self.set_square(2, state.char_at(1));
      self.set_square(3, state.char_at(2));
      self.set_square(4, state.char_at(3));
      self.set_square(5, state.char_at(4));
      self.set_square(6, state.char_at(5));
      self.set_square(7, state.char_at(6));
      self.set_square(8, state.char_at(7));
      self.set_square(9, state.char_at(8));
    }
  }

  /*
  pub fn as_string(&self) -> &str {
    let sliced = [
      self.who_has_it(9) as u8,
      self.who_has_it(8) as u8,
      self.who_has_it(7) as u8,
      self.who_has_it(6) as u8,
      self.who_has_it(5) as u8,
      self.who_has_it(4) as u8,
      self.who_has_it(3) as u8,
      self.who_has_it(2) as u8,
      self.who_has_it(1) as u8
    ].as_slice();
    str::from_utf8(sliced).unwrap().as_slice()
  }
  */
}

