use std::num::ParseIntError;
use boolinator::Boolinator;

#[derive(Debug, thiserror::Error)]
enum MyParseError {
    #[error("Input string is too long {0}")]
    InputTooLong(usize),
    #[error("Failed to parse {0}")]
    ParseFail(#[from] ParseIntError),
    #[error("Value is 0")]
    ValueIs0,
}

fn parse_item(value: &str) -> Result<i32, MyParseError> {
    if value.len() > 4 {
        return Err(MyParseError::InputTooLong(value.len()));
    }

    let result = value.parse()?;
    if result == 0 {
        Err(MyParseError::ValueIs0)
    } else {
        Ok(result)
    }
}

fn parse_items(items: &[(bool, &str)]) -> Result<Vec<(bool, i32)>, MainAppError> {
    items
        .iter()
        .cloned()
        .map(|(attr, input)| {
            Ok((
                attr,
                parse_item(input).map_err(|error| MainAppError::FailedToParse {
                    input: input.to_string(),
                    error,
                })?,
            ))
        })
        .collect()
}

#[derive(Debug, thiserror::Error)]
enum MyAnalyseError {
    #[error("Item should be divisible by 2 but it is not")]
    NotDivisibleBy2,
    #[error("Item should not be divisible by 2 but it is")]
    DivisibleBy2,
}

fn analyse_item((attr, value): (bool, i32)) -> Result<(), MyAnalyseError> {
    if attr {
        ((value % 2) == 0).ok_or(MyAnalyseError::NotDivisibleBy2)
    } else {
        ((value % 2) == 1).ok_or(MyAnalyseError::DivisibleBy2)
    }
}

fn analyse_items(items: &[(bool, i32)]) -> Result<(), MainAppError> {
    for item in items {
        if let Err(error) = analyse_item(*item) {
            return Err(MainAppError::FailedToAnalyse {
                attr: item.0,
                value: item.1,
                error,
            });
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MainAppError {
    #[error("Failed to parse {input} because {error}")]
    FailedToParse { input: String, error: MyParseError },
    #[error("Analysis failed for ({attr},{value}) because {error}")]
    FailedToAnalyse {
        attr: bool,
        value: i32,
        error: MyAnalyseError,
    },
}

fn run_app(input: Vec<(bool, &str)>) -> Result<(), MainAppError> {
    let parsed = parse_items(&input)?;
    analyse_items(&parsed)
}

fn main() {
    println!("1. {}", run_app(vec![(true, "0")]).unwrap_err());
    println!("2. {}", run_app(vec![(true, "a")]).unwrap_err());
    println!("3. {}", run_app(vec![(true, "10000")]).unwrap_err());
    println!(
        "4. {}",
        run_app(vec![(false, "1"), (true, "1")]).unwrap_err()
    );
    println!(
        "5. {}",
        run_app(vec![(false, "2"), (true, "2")]).unwrap_err()
    );
}
