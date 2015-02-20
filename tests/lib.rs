
extern crate rustactoe;

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
  
  board.from_string(" xxoox o ");
  let move_list = board.get_open_positions();
  assert_eq!(move_list.len(), 3);
  assert_eq!(move_list[0], 1);
  assert_eq!(move_list[1], 7);
  assert_eq!(move_list[2], 9);
}

#[test]
fn tttboard_clone() {
  let mut b0 : rustactoe::TttBoardBinRep = std::default::Default::default();
  b0.from_string("xoooxox  ");
  let mut b1 = b0.clone();
  assert_eq!(b0.as_string(), b1.as_string());
  b0.set_square(3, 'x');
  assert_eq!(b0.as_string(), "xoxoxox__");
  assert_eq!(b1.as_string(), "xoooxox__");

  b1.set_square(8, 'o');
  assert_eq!(b1.as_string(), "xoooxoxo_");
}

#[test]
fn tttboard_score() {
  let mut b0 : rustactoe::TttBoardBinRep = std::default::Default::default();
  b0.from_string("xoooxoxo ");
  assert_eq!(b0.score_single_board('x', 'o'), 0);

  b0.from_string("o xx  xoo");
  assert_eq!(b0.winner(), '_');
  let move_list = b0.get_open_positions();
  assert_eq!(move_list.len(), 3);
  assert_eq!(move_list[0], 2);
  assert_eq!(move_list[1], 5);
  assert_eq!(move_list[2], 6);

  /*
  let winning_move_list = b0.winning_move_list('x', 'o');
  assert_eq!(winning_move_list.len(), 3);
  assert_eq!(winning_move_list[0], 2);
  assert_eq!(winning_move_list[1], 5);
  assert_eq!(winning_move_list[2], 6);
  */

  b0.from_string("o xx  xoo");
  let winning_move_list = b0.winning_move_list('x', 'o');
  assert_eq!(winning_move_list.len(), 1);
  assert_eq!(winning_move_list[0], 5);

  let next_move_list = b0.non_winning_move_list('x', 'o');
  assert_eq!(next_move_list.len(), 2);
  assert_eq!(next_move_list[0], 2);
  assert_eq!(next_move_list[1], 6);

  b0.iterate('x', 'o');
}

#[test]
fn tttboard_minimax_score() {
  let mut b : rustactoe::TttBoardBinRep = std::default::Default::default();
  b.from_string("xxxoo    ");
  assert_eq!(b.winner(), 'x');
  assert_eq!(b.minimax_score('x', 'o'), 6);

  b.from_string("xx oo    ");
  assert_eq!(b.winner(), '_');
  assert_eq!(b.minimax_score('x', 'o'), 6);
}

