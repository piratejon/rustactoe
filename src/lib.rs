
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
  pub fn count_blanks(&self) -> usize { self.squares.len() }
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
    self.squares[index-1].set_value(value);
  }

  pub fn winner(&self) -> char {
    'x'
  }
}

