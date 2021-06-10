use std::ops::{Deref, DerefMut};

use serde::Serialize;

pub const NAME: &str = "$__v8_magic_u16string";
pub const FIELD_PTR: &str = "$__v8_magic_u16string_ptr";
pub const FIELD_LEN: &str = "$__v8_magic_u16string_len";

pub struct U16String(Vec<u16>);

impl U16String {
  pub fn with_zeroes(length: usize) -> U16String {
    U16String(vec![0u16; length])
  }

  pub fn truncate(&mut self, new_length: usize) {
    self.0.truncate(new_length);
    self.0.shrink_to_fit()
  }
}

impl Deref for U16String {
  type Target = [u16];
  fn deref(&self) -> &[u16] {
    &self.0
  }
}

impl DerefMut for U16String {
  fn deref_mut(&mut self) -> &mut [u16] {
    &mut self.0
  }
}

impl AsRef<[u16]> for U16String {
  fn as_ref(&self) -> &[u16] {
    &self.0
  }
}

impl AsMut<[u16]> for U16String {
  fn as_mut(&mut self) -> &mut [u16] {
    &mut self.0
  }
}

impl Serialize for U16String {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let mut s = serializer.serialize_struct(NAME, 3)?;
    s.serialize_field(FIELD_PTR, &(self.as_ptr() as usize))?;
    s.serialize_field(FIELD_LEN, &self.len())?;
    s.end()
  }
}
