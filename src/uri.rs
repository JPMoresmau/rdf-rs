// todo: implement

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uri {
  uri: String
}

// todo
impl Uri {
  pub fn new(uri: String) -> Uri {
    Uri {
      uri: uri
    }
  }

  // todo: rename to to_string()
  pub fn uri(&self) -> &String {
    &self.uri
  }
}