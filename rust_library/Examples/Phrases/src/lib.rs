pub mod greetings
{
  pub mod english;
  pub mod french
  {
    pub fn hello() -> String { return "bonjour".to_string(); }
    pub fn goodbye() -> String { return "au revoir".to_string(); }
  }
}

