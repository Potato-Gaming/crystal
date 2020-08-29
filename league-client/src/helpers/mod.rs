pub trait JoinIterator {
  fn join(self, sep: &str) -> String;
}

impl<I, T> JoinIterator for I
where
  I: IntoIterator<Item = T>,
  T: std::fmt::Display,
{
  fn join(self, sep: &str) -> String {
    use std::fmt::Write;

    let mut it = self.into_iter();
    let first = it.next().map(|f| f.to_string()).unwrap_or_default();

    it.fold(first, |mut acc, s| {
      write!(acc, "{}{}", sep, s).expect("Writing in a String shouldn't fail");
      acc
    })
  }
}
