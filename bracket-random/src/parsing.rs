use std::convert::From;
use std::error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

// Describes a dice roll type
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DiceType {
    pub n_dice: i32,
    pub die_type: i32,
    pub bonus: i32,
}

impl DiceType {
    pub fn new(n_dice: i32, die_type: i32, bonus: i32) -> Self {
        DiceType {
            n_dice,
            die_type,
            bonus,
        }
    }
}

impl Default for DiceType {
    fn default() -> DiceType {
        DiceType {
            n_dice: 0,
            die_type: 0,
            bonus: 0,
        }
    }
}

#[cfg(feature = "parsing")]
impl FromStr for DiceType {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_dice_string(s)
    }
}

#[cfg(feature = "parsing")]
impl From<&str> for DiceType {
    fn from(text: &str) -> Self {
        match parse_dice_string(text) {
            Ok(d) => d,
            Err(text) => panic!("Failed to convert dice string: {}", text.0),
        }
    }
}

impl fmt::Display for DiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n_dice {
            0 => write!(f, "{}", self.bonus),
            _x => match self.bonus {
                0 => write!(f, "{}d{}", self.n_dice, self.die_type),
                _x if _x > 0 => write!(f, "{}d{}+{}", self.n_dice, self.die_type, self.bonus),
                _y => write!(f, "{}d{}{}", self.n_dice, self.die_type, self.bonus),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiceParseError(String);

impl std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid dice string: {}", self.0)
    }
}

impl error::Error for DiceParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[allow(dead_code)]
// Parses a dice string, of the type "1d6+3", "3d8-4" or "1d20".
// It also parses ranges like "8-12" or constants like "9".
#[cfg(feature = "parsing")]
pub fn parse_dice_string(text: &str) -> Result<DiceType, DiceParseError> {
    let mut dice = text.trim();
    if dice.contains(&['d', 'D']) {
        let mut plus = 0_i32;

        if dice.contains("-") {
            let mut split = dice.split("-").map(|p| p.trim());
            dice = split.next().unwrap();
            plus = 0 - split.next().unwrap_or("0").parse().unwrap_or(0);
        } else if dice.contains("+") {
            let mut split = dice.split("+").map(|p| p.trim());
            dice = split.next().unwrap();
            plus = split.next().unwrap_or("0").parse().unwrap_or(0);
        }

        let parts: Vec<&str> = dice.split(&['d', 'D']).map(|p| p.trim()).collect();
        if parts.len() == 1 {
            return Err(DiceParseError(dice.to_string()));
        } else if parts.len() == 2 {
            let count = match parts[0].len() {
                0 => 1_i32,
                _ => match parts[0].parse() {
                    Ok(v) => v,
                    Err(_) => return Err(DiceParseError(dice.to_string())),
                },
            };
            let sides = parts[1].parse().unwrap_or(0_i32);
            return Ok(DiceType::new(count, sides, plus));
        }
    } else if dice.contains("-") && !dice.starts_with("-") {
        let mut split = dice.split("-").map(|p| p.trim());
        let low: i32 = split.next().unwrap().trim().parse().unwrap_or(0);
        let hi: i32 = split.next().unwrap().trim().parse().unwrap_or(0);

        let count = 1;
        let sides = hi - low + 1;
        let plus = low as i32 - 1;
        return Ok(DiceType::new(count, sides, plus));
    } else {
        let plus = match dice.trim().parse() {
            Ok(v) => v,
            Err(_) => return Err(DiceParseError(dice.to_string())),
        };
        return Ok(DiceType::new(0, 0, plus));
    }
    Err(DiceParseError(dice.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{parse_dice_string, DiceType};

    #[test]
    fn parse_1d6() {
        assert_eq!(parse_dice_string("1d6").unwrap(), DiceType::new(1, 6, 0));
        assert_eq!(parse_dice_string("1 d6").unwrap(), DiceType::new(1, 6, 0));
        assert_eq!(parse_dice_string("1D6").unwrap(), DiceType::new(1, 6, 0));
        assert_eq!(parse_dice_string("1D 6").unwrap(), DiceType::new(1, 6, 0));
    }

    #[test]
    fn parse_const() {
        assert_eq!(parse_dice_string("6").unwrap(), DiceType::new(0, 0, 6));
        assert_eq!(parse_dice_string("16").unwrap(), DiceType::new(0, 0, 16));
        assert_eq!(parse_dice_string("-6").unwrap(), DiceType::new(0, 0, -6));
    }

    #[test]
    fn parse_range() {
        assert_eq!(parse_dice_string("6-10").unwrap(), DiceType::new(1, 5, 5));
        assert_eq!(parse_dice_string("1-16").unwrap(), DiceType::new(1, 16, 0));
        assert_eq!(parse_dice_string("0-6").unwrap(), DiceType::new(1, 7, -1));
    }

    #[test]
    fn parse_1d20plus4() {
        assert_eq!(
            parse_dice_string("1d20+4").unwrap(),
            DiceType::new(1, 20, 4)
        );
        assert_eq!(
            parse_dice_string("1 D 20 -4").unwrap(),
            DiceType::new(1, 20, -4)
        );
    }

    #[test]
    fn parse_3d6minus2() {
        assert_eq!(parse_dice_string("3d6-2").unwrap(), DiceType::new(3, 6, -2));
    }

    #[test]
    fn parse_whitespace_test() {
        assert_eq!(
            parse_dice_string("3d6 - 2").unwrap(),
            DiceType::new(3, 6, -2)
        );
        assert_eq!(
            parse_dice_string(" 3d6- 2").unwrap(),
            DiceType::new(3, 6, -2)
        );
        assert_eq!(
            parse_dice_string("3 d 6- 2").unwrap(),
            DiceType::new(3, 6, -2)
        );
    }

    #[test]
    fn fail_parsing() {
        assert!(parse_dice_string("blah").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_parsing() {
        use serde_crate::{Deserialize, Serialize};
        let d = parse_dice_string("3d6 - 2").unwrap();
        let serialized = serde_json::to_string(&d).unwrap();
        let deserialized: DiceType = serde_json::from_str(&serialized).unwrap();
        assert!(d == deserialized);
    }
}
