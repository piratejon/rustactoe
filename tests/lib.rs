
extern crate rustactoe;

#[test]
fn test_it() {
  assert!(true);
}

#[test]
fn tttsquare() {
  let mut square = rustactoe::TttSquare { value: 'x' };
  square.set_x();
  assert_eq!(square.get_value(), 'x');
}

