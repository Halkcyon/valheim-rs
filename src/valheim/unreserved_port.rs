use std::{error, fmt, str::FromStr};

#[derive(Debug)]
pub struct UnreservedPort(u16);

impl FromStr for UnreservedPort {
    type Err = ParsePortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<u16>() {
            Ok(num) => {
                if num < 1 << 10 {
                    Err(ParsePortError(format!("Value '{}' is a reserved port.", num)))
                } else {
                    Ok(Self(num))
                }
            }
            Err(err) => Err(ParsePortError(format!("Unable to parse value '{}': {}", s, err)))
        }
    }
}

impl Default for UnreservedPort {
    fn default() -> Self {
        Self(2456)
    }
}

impl fmt::Display for UnreservedPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ParsePortError(String);

impl fmt::Display for ParsePortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for ParsePortError {}
