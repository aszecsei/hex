use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ValueIncorrectError {
  #[error("the character {0:?} is not a number")]
  NotNumber(char),
  #[error("no value")]
  NoValue,
}

#[derive(Debug, Clone, Error)]
pub struct UnitIncorrectError {
  pub character: char,
  pub expected_characters: &'static [char],
  pub also_expect_no_character: bool,
}
impl Display for UnitIncorrectError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    let expected_characters_length = self.expected_characters.len();
    if expected_characters_length == 0 {
      write!(
        f,
        "The character {:?} is incorrect. No character is expected.",
        self.character
      )?;
    } else if expected_characters_length == 1 {
      write!(
        f,
        "The character {:?} is incorrect. {:?}",
        self.character, self.expected_characters[0]
      )?;
      if self.also_expect_no_character {
        write!(f, " or no character is expected.")?;
      } else {
        write!(f, " is expected.")?;
      }
    } else {
      write!(
        f,
        "The character {:?} is incorrect. {:?}",
        self.character, self.expected_characters[0]
      )?;
      if expected_characters_length > 1 {
        for c in self.expected_characters[1..]
          .iter()
          .take(expected_characters_length - 2)
        {
          write!(f, ", {:?}", c)?;
        }
      }

      if self.also_expect_no_character {
        write!(
          f,
          ", {:?} or no character is expected.",
          self.expected_characters[expected_characters_length - 1]
        )?;
      } else {
        write!(
          f,
          " or {:?} is expected.",
          self.expected_characters[expected_characters_length - 1]
        )?;
      }
    }

    Ok(())
  }
}

#[derive(Debug, Clone, Error)]
pub enum ByteError {
  #[error(transparent)]
  ValueIncorrect(#[from] ValueIncorrectError),
  #[error(transparent)]
  UnitIncorrect(#[from] UnitIncorrectError),
}
