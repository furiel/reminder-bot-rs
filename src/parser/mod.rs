#[derive(PartialEq, Debug)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Later {
        amount: u64,
        unit: Unit,
        message: String,
    },
    At {
        date: String,
        message: String,
    },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ParseError,
    UnknownCommand,
    NotImplemented,
}

fn parse_amount(input: &str) -> Option<(u64, Unit)> {
    static UNITS: [char; 4] = ['s', 'm', 'h', 'd'];

    let last_char = input.chars().last()?;

    if last_char.is_numeric() {
        let unit = Unit::Seconds;
        let amount: u64 = input.parse().ok()?;

        return Some((amount, unit));
    }

    if UNITS.contains(&last_char) {
        let amount: u64 = {
            let mut numeric_part = String::from(input);
            numeric_part.pop();
            numeric_part.as_str().parse().ok()?
        };

        let unit: Unit = match last_char {
            's' => Unit::Seconds,
            'm' => Unit::Minutes,
            'h' => Unit::Hours,
            'd' => Unit::Days,
            _ => return None,
        };

        return Some((amount, unit));
    };

    None
}

fn parse_later(input: &str) -> Result<Command> {
    let mut splitter = input.splitn(2, ' ');

    let Some(amount_part) = splitter.next() else {
        return Err(Error::ParseError);
    };

    let Some((amount, unit)) = parse_amount(amount_part) else {
        return Err(Error::ParseError);
    };

    let Some(message) = splitter.next() else {
        return Err(Error::ParseError);
    };

    Ok(Command::Later {
        amount: amount,
        unit: unit,
        message: message.to_string(),
    })
}

fn parse_at(_input: &str) -> Result<Command> {
    Err(Error::NotImplemented)
}

fn extract_command(input: &str) -> Option<(&str, &str)> {
    let mut splitter = input.splitn(2, ' ');
    let command = splitter.next()?;
    let message = splitter.next()?;
    Some((command, message))
}

pub fn parse(input: &str) -> Result<Command> {
    let Some((command, rest)) = extract_command(input) else {
        return Err(Error::ParseError);
    };

    match command {
        "/later" => parse_later(rest),
        "/at" => parse_at(rest),
        _ => Err(Error::UnknownCommand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_amount_test() {
        assert_eq!(parse_amount("1"), Some((1, Unit::Seconds)));
        assert_eq!(parse_amount("1s"), Some((1, Unit::Seconds)));
        assert_eq!(parse_amount("2s"), Some((2, Unit::Seconds)));
        assert_eq!(parse_amount("2m"), Some((2, Unit::Minutes)));
        assert_eq!(parse_amount("3h"), Some((3, Unit::Hours)));
        assert_eq!(parse_amount("4d"), Some((4, Unit::Days)));

        assert_eq!(parse_amount(""), None);
        assert_eq!(parse_amount("d"), None);
        assert_eq!(parse_amount("1f"), None);
        assert_eq!(parse_amount("1dd"), None);
        assert_eq!(parse_amount("1d1"), None);
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("/later 2h message 1"),
            Ok(Command::Later {
                amount: 2,
                unit: Unit::Hours,
                message: "message 1".to_string(),
            })
        );

        assert_eq!(
            parse("/at 2023-07-25 message 1"),
            Err(Error::NotImplemented)
        );
        assert_eq!(parse("/not-exist"), Err(Error::ParseError));
        assert_eq!(parse("/later 1s"), Err(Error::ParseError));
        assert_eq!(parse("/later message"), Err(Error::ParseError));
    }
}
