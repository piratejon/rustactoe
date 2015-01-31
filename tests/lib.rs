
extern crate rustactoe;

#[test]
fn test_it() {
  assert!(true);
}

#[test]
fn tttsquare() {
  let mut square : rustactoe::TttSquare = std::default::Default::default();
  assert_eq!(square.get_value(), 'x');
  square.set_x();
  assert_eq!(square.get_value(), 'x');
  square.set_blank();
  assert_eq!(square.get_value(), '_');
  square.set_o();
  assert_eq!(square.get_value(), 'o');

  square.set_value('x');
  assert_eq!(square.get_value(), 'x');
  square.set_value('o');
  assert_eq!(square.get_value(), 'o');
  square.set_value('O');
  assert_eq!(square.get_value(), 'o');
  square.set_value('0');
  assert_eq!(square.get_value(), '_');
  square.set_value('_');
  assert_eq!(square.get_value(), '_');
  square.set_value('X');
  assert_eq!(square.get_value(), 'x');
  square.set_value('*');
  assert_eq!(square.get_value(), '_');
  square.set_value('9');
  assert_eq!(square.get_value(), '_');
}

#[test]
fn tttboard() {
  let mut board : rustactoe::TttBoard = std::default::Default::default();
  assert_eq!(board.count_blanks(), 9);

  assert_eq!(board.get_square(1), '_');
  assert_eq!(board.get_square(2), '_');
  assert_eq!(board.get_square(3), '_');
  assert_eq!(board.get_square(4), '_');
  assert_eq!(board.get_square(5), '_');
  assert_eq!(board.get_square(6), '_');
  assert_eq!(board.get_square(7), '_');
  assert_eq!(board.get_square(8), '_');
  assert_eq!(board.get_square(9), '_');

  board.set_square(8, 'x');
  assert_eq!(board.get_square(8), 'x');
  assert_eq!(board.count_blanks(), 8);

  board.set_square(1, ' ');
  assert_eq!(board.get_square(1), '_');
    /*
       1 | 2 | 3
       ---------
       4 | 5 | 6
       ---------
       7 | 8 | 9
       */
  board.set_square(9, 'x');
  assert_eq!(board.count_blanks(), 7);
  assert_eq!(board.winner(), '_');
  board.set_square(7, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'x');

  board.reset();
  assert_eq!(board.count_blanks(), 9);
  assert_eq!(board.get_square(1), '_');
  assert_eq!(board.get_square(2), '_');
  assert_eq!(board.get_square(3), '_');
  assert_eq!(board.get_square(4), '_');
  assert_eq!(board.get_square(5), '_');
  assert_eq!(board.get_square(6), '_');
  assert_eq!(board.get_square(7), '_');
  assert_eq!(board.get_square(8), '_');
  assert_eq!(board.get_square(9), '_');

  board.set_square(1, 'o');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.winner(), '_');
  board.set_square(5, 'o');
  assert_eq!(board.count_blanks(), 7);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'o');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'o');

  board.set_square(1, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(5, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'x');
}

#[test]
fn tttboard_binrep() {
  let mut board : rustactoe::TttBoardBinRep = std::default::Default::default();
  assert_eq!(board.count_blanks(), 9);

  assert_eq!(board.get_square(1), '_');
  assert_eq!(board.get_square(2), '_');
  assert_eq!(board.get_square(3), '_');
  assert_eq!(board.get_square(4), '_');
  assert_eq!(board.get_square(5), '_');
  assert_eq!(board.get_square(6), '_');
  assert_eq!(board.get_square(7), '_');
  assert_eq!(board.get_square(8), '_');
  assert_eq!(board.get_square(9), '_');

  // board.set_x(8);
  board.set_square(8, 'x');
  assert_eq!(board.get_square(8), 'x');
  assert_eq!(board.count_blanks(), 8);

  board.set_square(1, 'x');
  assert_eq!(board.get_square(1), 'x');
  assert_eq!(board.count_blanks(), 7);

  board.set_blank(1);
  board.set_square(1, ' ');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.get_square(1), '_');
  board.set_blank(1);
  board.set_square(1, ' ');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.get_square(1), '_');

  board.set_square(1, 'x');
  assert_eq!(board.get_square(1), 'x');
  assert_eq!(board.count_blanks(), 7);

  board.set_square(1, ' ');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.get_square(1), '_');
    /*
       1 | 2 | 3
       ---------
       4 | 5 | 6
       ---------
       7 | 8 | 9
       */
  board.set_square(9, 'x');
  assert_eq!(board.count_blanks(), 7);
  // assert_eq!(board.winner(), '_');
  board.set_square(7, 'x');
  assert_eq!(board.count_blanks(), 6);
  // assert_eq!(board.winner(), 'x');

  /*
  board.reset();
  assert_eq!(board.count_blanks(), 9);
  assert_eq!(board.get_square(1), '_');
  assert_eq!(board.get_square(2), '_');
  assert_eq!(board.get_square(3), '_');
  assert_eq!(board.get_square(4), '_');
  assert_eq!(board.get_square(5), '_');
  assert_eq!(board.get_square(6), '_');
  assert_eq!(board.get_square(7), '_');
  assert_eq!(board.get_square(8), '_');
  assert_eq!(board.get_square(9), '_');

  board.set_square(1, 'o');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.winner(), '_');
  board.set_square(5, 'o');
  assert_eq!(board.count_blanks(), 7);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'o');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'o');

  board.set_square(1, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(5, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'x');
  */
}

