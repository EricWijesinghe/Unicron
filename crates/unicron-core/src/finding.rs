/// A single security finding discovered during an assessment.
use crate::severity::Severity;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub title: String,
    pub severity: Severity,
    pub description: String,
    pub location: String,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_carries_its_severity() {
        let finding = Finding {
            title: String::from("Open SSH port"),
            severity: Severity::Medium,
            description: String::from("Port 22 is exposed to the internet"),
            location: String::from("10.0.0.5:22"),
        };
        assert_eq!(finding.severity, Severity::Medium);
        assert!(finding.severity < Severity::High);
    }
}
