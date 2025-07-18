//! we should properly bubble up errors from components

use dioxus::prelude::*;
use dioxus_core::generation;

fn app() -> Element {
    let raw = match generation() % 2 {
        0 => "123.123",
        1 => "123.123.123",
        _ => unreachable!(),
    };

    let value = raw.parse::<f32>().unwrap_or(123.123);

    rsx! { div { "hello {value}" } }
}

#[test]
fn bubbles_error() {
    let mut dom = VirtualDom::new(app);

    {
        let _edits = dom.rebuild_to_vec();
    }

    dom.mark_dirty(ScopeId::APP);

    _ = dom.render_immediate_to_vec();
}
