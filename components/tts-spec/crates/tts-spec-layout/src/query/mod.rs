//! Functions for querying timeline properties.

use crate::events::{TimelineEvent, TimelineEventKind};

/// Estimate total silence duration in the timeline.
pub fn estimate_silence_ms(events: &[TimelineEvent]) -> u64 {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            TimelineEventKind::Silence { duration_ms } => Some(*duration_ms as u64),
            _ => None,
        })
        .sum()
}
