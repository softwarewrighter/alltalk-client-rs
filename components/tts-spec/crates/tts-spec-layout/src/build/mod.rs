//! Functions for building timeline events from scripts.

use crate::events::{TimelineEvent, TimelineEventKind};
use tts_spec_model::{Script, SegmentId};

/// Build a timeline of events from a script.
pub fn build_timeline(script: &Script) -> Vec<TimelineEvent> {
    let mut events = Vec::new();
    let mut current_ms: u64 = 0;
    let mut order_index: u32 = 0;

    for seg in &script.segments {
        push_events_for_segment(&mut events, &mut current_ms, &mut order_index, seg);
    }

    events
}

/// Push timeline events for a single segment.
fn push_events_for_segment(
    events: &mut Vec<TimelineEvent>,
    current_ms: &mut u64,
    order_index: &mut u32,
    seg: &tts_spec_model::Segment,
) {
    if seg.pause_before_ms > 0 {
        push_silence(events, current_ms, order_index, seg.pause_before_ms);
    }

    push_tts_segment(events, current_ms, order_index, seg.id.clone());

    if seg.pause_after_ms > 0 {
        push_silence(events, current_ms, order_index, seg.pause_after_ms);
    }
}

/// Push a silence event onto the timeline.
fn push_silence(
    events: &mut Vec<TimelineEvent>,
    current_ms: &mut u64,
    order_index: &mut u32,
    duration_ms: u32,
) {
    events.push(TimelineEvent {
        order_index: *order_index,
        at_ms: *current_ms,
        kind: TimelineEventKind::Silence { duration_ms },
    });
    *current_ms += duration_ms as u64;
    *order_index += 1;
}

/// Push a TTS segment event onto the timeline.
fn push_tts_segment(
    events: &mut Vec<TimelineEvent>,
    current_ms: &mut u64,
    order_index: &mut u32,
    segment_id: SegmentId,
) {
    events.push(TimelineEvent {
        order_index: *order_index,
        at_ms: *current_ms,
        kind: TimelineEventKind::TtsSegment { segment_id },
    });
    *order_index += 1;
}
