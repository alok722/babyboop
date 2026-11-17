//! Command-line switches: `--selftest` and `--exit-after <seconds>`.

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Config {
    pub selftest: bool,
    pub exit_after: Option<f64>,
}

impl Config {
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Self {
        let args: Vec<String> = args.into_iter().collect();
        let selftest = args.iter().any(|a| a == "--selftest");
        let exit_after = args
            .iter()
            .position(|a| a == "--exit-after")
            .and_then(|i| args.get(i + 1))
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|s| s.is_finite() && *s >= 0.0);
        Config { selftest, exit_after }
    }

    pub fn test_mode(&self) -> bool {
        self.selftest || self.exit_after.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Config {
        Config::parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn no_args_is_normal_mode() {
        let c = parse(&["BabyBoop.exe"]);
        assert_eq!(c, Config::default());
        assert!(!c.test_mode());
    }

    #[test]
    fn selftest_flag() {
        assert!(parse(&["x", "--selftest"]).selftest);
    }

    #[test]
    fn exit_after_uses_a_dot_decimal() {
        assert_eq!(parse(&["x", "--exit-after", "2.5"]).exit_after, Some(2.5));
    }

    #[test]
    fn exit_after_without_value_is_ignored() {
        assert_eq!(parse(&["x", "--exit-after"]).exit_after, None);
    }

    #[test]
    fn exit_after_rejects_junk() {
        assert_eq!(parse(&["x", "--exit-after", "2,5"]).exit_after, None);
        assert_eq!(parse(&["x", "--exit-after", "-1"]).exit_after, None);
        assert_eq!(parse(&["x", "--exit-after", "inf"]).exit_after, None);
    }
}
