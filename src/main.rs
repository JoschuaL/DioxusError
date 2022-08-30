use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let atom = use_read(&cx, ATOM);

    cx.render(rsx! {
        Component{},
    })
}

pub struct S {
    pub x: String,
}

pub static ATOM: Atom<Vec<S>> = |_| Vec::new();
pub static OTHER_ATOM: Atom<Vec<String>> = |_| Vec::new();

pub fn Component(cx: Scope) -> Element {
    let other_atom = use_read(&cx, OTHER_ATOM);
    None
}
