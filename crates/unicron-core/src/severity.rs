/// The severity level of a security finding, from least to most serious.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_is_more_severe_than_low() {
        assert!(Severity::Critical > Severity::Low);
        assert!(Severity::High > Severity::Medium);
    }
}