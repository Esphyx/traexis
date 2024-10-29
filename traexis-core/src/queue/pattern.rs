#[derive(Debug)]
pub struct Pattern {
    pub multiset: std::collections::HashMap<super::Tetromino, usize>,
    pub amount: usize,
}

impl Pattern {
    #[inline]
    pub fn size(&self) -> usize {
        self.multiset.values().sum()
    }
    pub fn next(&mut self) -> super::Tetromino {
        let total_weight = self.size();
        let random_value = rand::Rng::gen_range(&mut rand::thread_rng(), 0..total_weight);

        let mut cumulative = 0;
        for (&item, frequency) in self.multiset.iter_mut() {
            cumulative += *frequency;
            if cumulative < random_value {
                continue;
            }

            *frequency -= 1;
            if *frequency == 0 {
                self.multiset.remove_entry(&item);
            }

            self.amount -= 1;

            return item;
        }

        panic!("There are no next tetrominos!");
    }
}

impl super::Parsing for Pattern {
    fn parse<T: Into<String>>(value: T) -> Result<Self, String> {
        let input = value.into();

        let validation = regex::Regex::new(r"^((?<pattern>\[.+?\])|(?<all>\*))(p(?<amount>\d+))?$")
            .map_err(|_| "Invalid regex pattern!")?;

        let captures = validation
            .captures(input.as_str())
            .ok_or("Invalid pattern format!")?;

        let collection: Vec<_> = if captures.name("all").is_some() {
            <super::Tetromino as strum::IntoEnumIterator>::iter().collect()
        } else if let Some(capture) = captures.name("pattern") {
            capture.as_str()[1..capture.len() - 1]
                .chars()
                .map(super::Tetromino::parse)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            return Err(String::from("Invalid multiset!"));
        };

        let amount = captures
            .name("amount")
            .map_or(1, |r#match| r#match.as_str().parse::<usize>().unwrap_or(1));

        if amount > collection.len() {
            return Err(String::from(
                "Amount may not be greater than the number of tetrominos in the multiset!",
            ));
        }

        let mut multiset = std::collections::HashMap::new();
        for item in collection {
            *multiset.entry(item).or_insert(0) += 1;
        }

        Ok(Self { multiset, amount })
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        itertools::Itertools::sorted(self.multiset.iter())
            .eq(itertools::Itertools::sorted(other.multiset.iter()))
            && self.amount == other.amount
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<_> = self
            .multiset
            .iter()
            .map(|(tetromino, &n)| tetromino.to_string().repeat(n))
            .collect();
        write!(f, "[{}]p{}", items.join(""), self.amount)
    }
}
