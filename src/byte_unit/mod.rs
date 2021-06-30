mod error;
pub use error::*;

/// KB
pub const KILOBYTE: u128 = 1_000;
/// KiB
pub const KIBIBYTE: u128 = 1 << 10;
/// MB
pub const MEGABYTE: u128 = 1_000_000;
/// MiB
pub const MEBIBYTE: u128 = 1 << 20;
/// GB
pub const GIGABYTE: u128 = 1_000_000_000;
/// GiB
pub const GIBIBYTE: u128 = 1 << 30;
/// TB
pub const TERABYTE: u128 = 1_000_000_000_000;
/// TiB
pub const TEBIBYTE: u128 = 1 << 40;
/// PB
pub const PETABYTE: u128 = 1_000_000_000_000_000;
/// PiB
pub const PEBIBYTE: u128 = 1 << 50;
/// EB
pub const EXABYTE: u128 = 1_000_000_000_000_000_000;
/// EiB
pub const EXBIBYTE: u128 = 1 << 60;

/// Convert n KB to bytes.
#[inline]
pub const fn n_kb_bytes(bytes: u128) -> u128 {
    bytes * KILOBYTE
}
/// Convert n KiB to bytes.
#[inline]
pub const fn n_kib_bytes(bytes: u128) -> u128 {
    bytes * KIBIBYTE
}

/// Convert n MB to bytes.
#[inline]
pub const fn n_mb_bytes(bytes: u128) -> u128 {
    bytes * MEGABYTE
}
/// Convert n MiB to bytes.
#[inline]
pub const fn n_mib_bytes(bytes: u128) -> u128 {
    bytes * MEBIBYTE
}

/// Convert n GB to bytes.
#[inline]
pub const fn n_gb_bytes(bytes: u128) -> u128 {
    bytes * GIGABYTE
}
/// Convert n GiB to bytes.
#[inline]
pub const fn n_gib_bytes(bytes: u128) -> u128 {
    bytes * GIBIBYTE
}

/// Convert n TB to bytes.
#[inline]
pub const fn n_tb_bytes(bytes: u128) -> u128 {
    bytes * TERABYTE
}
/// Convert n TiB to bytes.
#[inline]
pub const fn n_tib_bytes(bytes: u128) -> u128 {
    bytes * TEBIBYTE
}

/// Convert n PB to bytes.
#[inline]
pub const fn n_pb_bytes(bytes: u128) -> u128 {
    bytes * PETABYTE
}
/// Convert n PiB to bytes.
#[inline]
pub const fn n_pib_bytes(bytes: u128) -> u128 {
    bytes * PEBIBYTE
}

/// Convert n EB to bytes.
#[inline]
pub const fn n_eb_bytes(bytes: u128) -> u128 {
    bytes * EXABYTE
}
/// Convert n EiB to bytes.
#[inline]
pub const fn n_eib_bytes(bytes: u128) -> u128 {
    bytes * EXBIBYTE
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ByteUnit {
    B,
    KB,
    KiB,
    MB,
    MiB,
    GB,
    GiB,
    TB,
    TiB,
    PB,
    PiB,
    EB,
    EiB,
    ZB,
    ZiB,
    YB,
    YiB,
}

fn get_char_from_bytes(e: u8, mut bytes: std::str::Bytes) -> char {
    use std::str::FromStr;
    let width = unsafe { utf8_width::get_width_assume_valid(e) };
    let mut char_bytes = [e; 4];
    if width > 1 {
        for e in char_bytes[1..].iter_mut().take(width - 1) {
            *e = bytes.next().unwrap();
        }
    }
    let char_str = unsafe { std::str::from_utf8_unchecked(&char_bytes[..width]) };
    char::from_str(char_str).unwrap()
}
fn read_xib(e: Option<u8>, mut bytes: std::str::Bytes) -> Result<ByteUnit, UnitIncorrectError> {
    match e {
        Some(e) => match e.to_ascii_uppercase() {
            b'B' => match bytes.next() {
                Some(e) => Err(UnitIncorrectError {
                    character: get_char_from_bytes(e, bytes),
                    expected_characters: &[],
                    also_expect_no_character: false,
                }),
                None => Ok(ByteUnit::B),
            },
            b'K' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::KiB)
                } else {
                    Ok(ByteUnit::KB)
                }
            }
            b'M' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::MiB)
                } else {
                    Ok(ByteUnit::MB)
                }
            }
            b'G' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::GiB)
                } else {
                    Ok(ByteUnit::GB)
                }
            }
            b'T' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::TiB)
                } else {
                    Ok(ByteUnit::TB)
                }
            }
            b'P' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::PiB)
                } else {
                    Ok(ByteUnit::PB)
                }
            }
            b'E' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::EiB)
                } else {
                    Ok(ByteUnit::EB)
                }
            }
            b'Z' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::ZiB)
                } else {
                    Ok(ByteUnit::ZB)
                }
            }
            b'Y' => {
                if read_ib(bytes)? {
                    Ok(ByteUnit::YiB)
                } else {
                    Ok(ByteUnit::YB)
                }
            }
            _ => Err(UnitIncorrectError {
                character: get_char_from_bytes(e, bytes),
                expected_characters: &['B', 'K', 'M', 'G', 'T', 'P', 'E', 'Z'],
                also_expect_no_character: true,
            }),
        },
        None => Ok(ByteUnit::B),
    }
}
fn read_ib(mut bytes: std::str::Bytes) -> Result<bool, UnitIncorrectError> {
    match bytes.next() {
        Some(e) => match e.to_ascii_uppercase() {
            b'I' => match bytes.next() {
                Some(e) => match e.to_ascii_uppercase() {
                    b'B' => Ok(true),
                    _ => Err(UnitIncorrectError {
                        character: get_char_from_bytes(e, bytes),
                        expected_characters: &['B'],
                        also_expect_no_character: true,
                    }),
                },
                None => Ok(true),
            },
            b'B' => match bytes.next() {
                Some(e) => Err(UnitIncorrectError {
                    character: get_char_from_bytes(e, bytes),
                    expected_characters: &[],
                    also_expect_no_character: false,
                }),
                None => Ok(false),
            },
            _ => Err(UnitIncorrectError {
                character: get_char_from_bytes(e, bytes),
                expected_characters: &['B', 'i'],
                also_expect_no_character: true,
            }),
        },
        None => Ok(true),
    }
}

pub struct Byte(u128);

impl Byte {
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Byte, ByteError> {
        let s = s.as_ref().trim();
        let mut bytes = s.bytes();
        let mut value = match bytes.next() {
            Some(e) => match e {
                b'0'..=b'9' => f64::from(e - b'0'),
                _ => {
                    return Err(
                        ValueIncorrectError::NotNumber(get_char_from_bytes(e, bytes)).into(),
                    )
                }
            },
            None => return Err(ValueIncorrectError::NoValue.into()),
        };

        let e = 'outer: loop {
            match bytes.next() {
                Some(e) => match e {
                    b'0'..=b'9' => {
                        value = value * 10.0 + f64::from(e - b'0');
                    }
                    b'.' => {
                        let mut i = 0.1;
                        loop {
                            match bytes.next() {
                                Some(e) => match e {
                                    b'0'..=b'9' => {
                                        value += f64::from(e - b'0') * i;
                                        i /= 10.0;
                                    }
                                    _ => {
                                        if (i * 10.0) as u8 == 1 {
                                            return Err(ValueIncorrectError::NotNumber(
                                                get_char_from_bytes(e, bytes),
                                            )
                                            .into());
                                        }
                                        match e {
                                            b' ' => loop {
                                                match bytes.next() {
                                                    Some(e) => match e {
                                                        b' ' => (),
                                                        _ => break 'outer Some(e),
                                                    },
                                                    None => break 'outer None,
                                                }
                                            },
                                            _ => break 'outer Some(e),
                                        }
                                    }
                                },
                                None => {
                                    if (i * 10.0) as u8 == 1 {
                                        return Err(ValueIncorrectError::NotNumber(
                                            get_char_from_bytes(e, bytes),
                                        )
                                        .into());
                                    }
                                    break 'outer None;
                                }
                            }
                        }
                    }
                    b' ' => loop {
                        match bytes.next() {
                            Some(e) => match e {
                                b' ' => (),
                                _ => break 'outer Some(e),
                            },
                            None => break 'outer None,
                        }
                    },
                    _ => break 'outer Some(e),
                },
                None => break None,
            }
        };

        let unit = read_xib(e, bytes)?;
        let bytes = get_bytes(value, unit);

        Ok(Byte(bytes))
    }
}

fn get_bytes(value: f64, unit: ByteUnit) -> u128 {
    match unit {
        ByteUnit::B => value as u128,
        ByteUnit::KB => (value * KILOBYTE as f64) as u128,
        ByteUnit::KiB => (value * KIBIBYTE as f64) as u128,
        ByteUnit::MB => (value * MEGABYTE as f64) as u128,
        ByteUnit::MiB => (value * MEBIBYTE as f64) as u128,
        ByteUnit::GB => n_kb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::GiB => n_kib_bytes((value * MEBIBYTE as f64) as u128),
        ByteUnit::TB => n_mb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::TiB => n_mib_bytes((value * MEBIBYTE as f64) as u128),
        ByteUnit::PB => n_gb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::PiB => n_gib_bytes((value * MEBIBYTE as f64) as u128),
        ByteUnit::EB => n_tb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::EiB => n_tib_bytes((value * MEBIBYTE as f64) as u128),
        ByteUnit::ZB => n_pb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::ZiB => n_pib_bytes((value * MEBIBYTE as f64) as u128),
        ByteUnit::YB => n_eb_bytes((value * MEGABYTE as f64) as u128),
        ByteUnit::YiB => n_eib_bytes((value * MEBIBYTE as f64) as u128),
    }
}

pub fn parse_bytes<S: AsRef<str>>(s: S) -> Result<u128, ByteError> {
    let b = Byte::from_str(s)?;
    Ok(b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes() {
        assert_eq!(parse_bytes("128").unwrap(), 128);
        assert_eq!(parse_bytes("128B").unwrap(), 128);
    }

    #[test]
    fn kilobytes() {
        assert_eq!(parse_bytes("128KB").unwrap(), 128_000);
        assert_eq!(parse_bytes("12.8  KB").unwrap(), 12_800);
    }

    #[test]
    fn kibibytes() {
        assert_eq!(parse_bytes("128KiB").unwrap(), 131_072);
        assert_eq!(parse_bytes("12.5  KiB").unwrap(), 12_800);
        assert_eq!(parse_bytes("1.25 K").unwrap(), 1_280);
    }
}
