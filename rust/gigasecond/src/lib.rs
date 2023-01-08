// use std::ops::Add;

use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(Duration::milliseconds(1_000_000_000 * 1_000))
}
