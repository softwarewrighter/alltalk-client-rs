//! Script editor and segment display components.

use tts_spec_model::{Script, Segment, SegmentId};
use yew::prelude::*;

/// Properties for ScriptEditor component.
#[derive(Properties, PartialEq)]
pub struct ScriptEditorProps {
    pub script: Script,
    pub selected: Option<SegmentId>,
    pub on_add: Callback<()>,
}

/// Script editor showing segment list with add button.
#[function_component(ScriptEditor)]
pub fn script_editor(props: &ScriptEditorProps) -> Html {
    let on_add = props.on_add.clone();

    html! {
        <div class="script-editor">
            <div class="segments-list">
                { for props.script.segments.iter().map(|seg| {
                    let selected = props.selected.as_ref() == Some(&seg.id);
                    html! { <SegmentCard segment={seg.clone()} {selected} /> }
                })}
            </div>
            <button class="btn btn-add" onclick={move |_| on_add.emit(())}>
                {"+ Add Segment"}
            </button>
        </div>
    }
}

/// Properties for SegmentCard component.
#[derive(Properties, PartialEq)]
pub struct SegmentCardProps {
    pub segment: Segment,
    pub selected: bool,
}

/// Individual segment card display.
#[function_component(SegmentCard)]
pub fn segment_card(props: &SegmentCardProps) -> Html {
    let class = if props.selected { "segment-card selected" } else { "segment-card" };
    let text = if props.segment.text.is_empty() {
        html! { <em>{"(empty)"}</em> }
    } else {
        html! { {&props.segment.text} }
    };

    html! {
        <div class={class}>
            <div class="segment-header">
                <span class="segment-id">{&props.segment.id.0}</span>
                <span class="segment-engine">{format!("{:?}", props.segment.engine)}</span>
            </div>
            <div class="segment-text">{text}</div>
        </div>
    }
}
