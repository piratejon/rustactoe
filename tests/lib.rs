
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
}

#[test]
fn tttboard() {
  let mut board : rustactoe::TttBoard = std::default::Default::default();
  assert_eq!(board.count_blanks(), 9);

  assert_eq!(board.get_square(1), '_');
}

