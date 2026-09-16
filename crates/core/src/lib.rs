//! Reusable application logic for the project skeleton.

/// Human-readable greeting used by the example binary.
pub const GREETING: &str = "Hello, ARGVUS Rust Skeleton!";

/// Returns the greeting exposed by the example application.
pub fn greeting() -> &'static str {
  GREETING
}

#[cfg(test)]
mod tests {
  use super::{GREETING, greeting};

  #[test]
  fn greeting_is_stable() {
    assert_eq!(greeting(), GREETING);
  }
}
