
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
  // assert_eq!(board.as_string(), "        x"); // 123456789
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

  // board.set_square(1, 'o');
  board.set_o(1);
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.winner(), '_');
  assert_eq!(board.get_square(5), '_');
  board.set_square(5, 'o');
  assert_eq!(board.count_blanks(), 7);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'o');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'o');

  board.set_square(5, '_');
  assert_eq!(board.count_blanks(), 7);

  board.set_square(5, 'o');
  assert_eq!(board.get_square(5), 'o');

  board.set_square(1, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(5, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'x');
  assert_eq!(board.count_blanks(), 6);
  assert_eq!(board.winner(), 'x');

  board.disable_x(9);
  assert_eq!(board.get_square(9), '_');
  assert_eq!(board.count_blanks(), 7);

  board.reset();
  assert_eq!(board.count_blanks(), 9);

}

#[test]
fn tttboard_state() {
  let mut board : rustactoe::TttBoardBinRep = std::default::Default::default();

  assert_eq!(board.count_blanks(), 9);
  board.set_square(5, 'x');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.get_square(5), 'x');
  board.set_square(5, 'o');
  assert_eq!(board.count_blanks(), 8);
  assert_eq!(board.get_square(5), 'o');
  board.set_square(5, '_');
  assert_eq!(board.count_blanks(), 9);
  assert_eq!(board.get_square(5), '_');
}

#[test]
fn tttboard_init() {
  let mut board : rustactoe::TttBoardBinRep = std::default::Default::default();
  board.from_string("xxooxo xx");
  assert_eq!(board.count_blanks(), 1);
  assert_eq!(board.get_square(1), 'x');
  assert_eq!(board.get_square(2), 'x');
  assert_eq!(board.get_square(3), 'o');
  assert_eq!(board.get_square(4), 'o');
  assert_eq!(board.get_square(5), 'x');
  assert_eq!(board.get_square(6), 'o');
  assert_eq!(board.get_square(7), '_');
  assert_eq!(board.get_square(8), 'x');
  assert_eq!(board.get_square(9), 'x');
  assert_eq!(board.as_string().len(), 9);
  assert_eq!(board.as_string(), "xxooxo_xx");

  board.from_string("xoooxox  ");
  assert_eq!(board.count_blanks(), 2);
  assert_eq!(board.get_square(1), 'x');
  assert_eq!(board.get_square(2), 'o');
  assert_eq!(board.get_square(3), 'o');
  assert_eq!(board.get_square(4), 'o');
  assert_eq!(board.get_square(5), 'x');
  assert_eq!(board.get_square(6), 'o');
  assert_eq!(board.get_square(7), 'x');
  assert_eq!(board.get_square(8), '_');
  assert_eq!(board.get_square(9), '_');
  assert_eq!(board.as_string(), "xoooxox__");
}

#[test]
fn tttboard_next_state() {
  let mut board : rustactoe::TttBoardBinRep = std::default::Default::default();

  board.from_string("xoooxox  ");
  assert_eq!(board.compute_x_next_move(), 9); // XXX trivial
  assert_eq!(board.winner(), '_');
  board.set_square(9, 'x');
  assert_eq!(board.winner(), 'x');

  board.from_string("xoooxox  ");
  let move_list = board.get_open_positions();
  assert_eq!(move_list.len(), 2);
  assert_eq!(move_list[0], 8);
  assert_eq!(move_list[1], 9);
}

