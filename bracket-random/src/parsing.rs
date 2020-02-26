use regex::Regex;
use std::error;
use std::fmt;

// Describes a dice roll type
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            n_dice: 1,
            die_type: 4,
            bonus: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiceParseError;

impl std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid dice string")
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
#[cfg(feature = "parsing")]
pub fn parse_dice_string(dice: &str) -> Result<DiceType, DiceParseError> {
    let dice = &dice.split_whitespace().collect::<Vec<_>>().join("");
    lazy_static! {
        static ref DICE_RE: Regex = Regex::new(r"(\d+)d(\d+)([\+\-]\d+)?").unwrap();
    }
    let mut result: DiceType = DiceType::default();
    let mut did_something = false;
    for cap in DICE_RE.captures_iter(dice) {
        did_something = true;
        if let Some(group) = cap.get(1) {
            match group.as_str().parse::<i32>() {
                Ok(number) => result.n_dice = number,
                Err(_) => return Err(DiceParseError {}),
            }
        } else {
            return Err(DiceParseError {});
        }
        if let Some(group) = cap.get(2) {
            match group.as_str().parse::<i32>() {
                Ok(number) => result.die_type = number,
                Err(_) => return Err(DiceParseError {}),
            }
        } else {
            return Err(DiceParseError {});
        }
        if let Some(group) = cap.get(3) {
            match group.as_str().parse::<i32>() {
                Ok(number) => result.bonus = number,
                Err(_) => return Err(DiceParseError {}),
            }
        }
    }
    if !did_something {
        return Err(DiceParseError {});
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{parse_dice_string, DiceType};

    #[test]
    fn parse_1d6() {
        assert_eq!(parse_dice_string("1d6").unwrap(), DiceType::new(1, 6, 0));
    }

    #[test]
    fn parse_1d20plus4() {
        assert_eq!(
            parse_dice_string("1d20+4").unwrap(),
            DiceType::new(1, 20, 4)
        );
    }

    #[test]
    fn parse_3d6minus2() {
        assert_eq!(parse_dice_string("3d6-2").unwrap(), DiceType::new(3, 6, -2));
    }

    #[test]
    fn parse_whitespace_test() {
        assert_eq!(parse_dice_string("3d6 - 2").unwrap(), DiceType::new(3, 6, -2));
        assert_eq!(parse_dice_string(" 3d6- 2").unwrap(), DiceType::new(3, 6, -2));
        assert_eq!(parse_dice_string("3 d 6- 2").unwrap(), DiceType::new(3, 6, -2));
    }

    #[test]
    fn fail_parsing() {
        assert!(parse_dice_string("blah").is_err());
    }
}
