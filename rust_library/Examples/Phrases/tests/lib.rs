#[cfg(test)]
mod tests
{
  extern crate phrases;

  #[test]
  #[should_panic]
  #[ignore]
  fn english_greeting_correct()
  {
    assert_eq!("helloo", phrases::greetings::english::hello());
  }

}