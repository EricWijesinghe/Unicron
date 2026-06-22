//! Scanning and assessment pillar for UNICRON.
use unicron_core::{Finding, Severity};

/// Produces a sample finding to prove the cross-crate wiring works.
pub fn simple_finding() -> Finding {
    Finding {
        title: String::from("Open SSH port"),
        severity: Severity::Critical,
        description: String::from("Port 22 is exposed to the internet"),
        location: String::from("10.0.0.5:22"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_finding_has_expected_severity() {
        let finding = simple_finding();
        assert_eq!(finding.severity, Severity::Critical);
    }
}
