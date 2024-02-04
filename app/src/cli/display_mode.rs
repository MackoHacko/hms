use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayMode {
    Large,
    Small,
}

impl ValueEnum for DisplayMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[DisplayMode::Large, DisplayMode::Small]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            DisplayMode::Large => PossibleValue::new("large").help("Full screen mode"),
            DisplayMode::Small => PossibleValue::new("small")
                .help("Small mode; draws gui starting from current cursor line"),
        })
    }
}

impl std::fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for DisplayMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid mode: {s}"))
    }
}
