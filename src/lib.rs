use seed::{*, prelude::*};


struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
        Msg::Decrement => model.val -= 1,
    }
}


fn view(model: &Model) -> impl View<Msg> {
    vec![
        h1!["This is Picard"],
        h2!["An Example Project with WASM"],
        p!["This is a Seed based Rust WASM example application."],
        button![
            simple_ev(Ev::Click, Msg::Increment),
            "+"
        ],
        button![
            simple_ev(Ev::Click, Msg::Decrement),
            "-"
        ],
        span![format!("Counter: {}", model.val)],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}