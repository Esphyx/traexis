use itertools::Itertools;
use pattern::Pattern;
use regex::Regex;
use tetromino::Tetromino;

pub mod pattern;
pub mod tetromino;

pub trait Parsing {
    fn parse<T: Into<String>>(value: T) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(PartialEq, Debug)]
pub struct Queue {
    pub sequence: Vec<pattern::Pattern>,
    pub hold: Option<tetromino::Tetromino>,
}

impl Queue {
    pub fn collapse(&mut self, _n: usize) {
        todo!()
    }
}

impl Parsing for Queue {
    fn parse<T: Into<String>>(value: T) -> Result<Self, String> {
        let input = value.into();

        let hold_pattern = r"^((?<hold>.):)?";
        let sequence_pattern = r"(?<pattern>((\[.+?\])|\*)(p\d+)?)";

        let hold = Regex::new(format!("{hold_pattern}{sequence_pattern}").as_str())
            .map_err(|_| "Invalid regex pattern!")?
            .captures(input.as_str())
            .ok_or("Invalid queue format!")?
            .name("hold")
            .map(|capture| Tetromino::parse(&capture.as_str()[..1]))
            .transpose()
            .map_err(|err| err + " at hold!")?;

        let mut sequence = Vec::new();
        for (i, captures) in Regex::new(&sequence_pattern)
            .map_err(|_| "Invalid regex pattern!")?
            .captures_iter(input.as_str())
            .enumerate()
        {
            if let Some(capture) = captures.name("pattern") {
                sequence.push(
                    Pattern::parse(capture.as_str())
                        .map_err(|err| format!("{err} at pattern {}!", i + 1))?,
                );
            }
        }

        Ok(Self { sequence, hold })
    }
}

impl std::fmt::Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &std::iter::once(
                self.hold
                    .as_ref()
                    .map_or(String::new(), |tetromino| format!("{}:", tetromino)),
            )
            .chain(self.sequence.iter().map(ToString::to_string))
            .join(""),
        )
    }
}
