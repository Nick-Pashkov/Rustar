use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::Utc;

mod node;
use node::Node;

mod astar;
mod grid;
use grid::Grid;

pub type Position2D = (i32, i32);

enum Msg {
    ToggleStart,
    SetDimensions(usize, i32),
    SetPlacingMode(PlaceMode),
    SetStart(Position2D),
    SetTarget(Position2D),
    SetTime(i64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeState {
    None,
    Start,
    Target,
    Open,
    Closed,
    Current,
    Path,
    Wall
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlaceMode {
    Start,
    Target,
    Wall,
    None
}

struct Model {
    is_started: bool,
    pub rows: i32,
    pub cols: i32,
    pub place_mode: PlaceMode,
    pos_start: Position2D,
    pos_target: Position2D,
    time_start: i64,
    time_end: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        /*let interval = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::Tick))
        };*/

        Self {
            is_started: false,
            rows: 10,
            cols: 10,
            place_mode: PlaceMode::None,
            pos_start: (0, 0),
            pos_target: (9, 9),
            time_start: 0,
            time_end: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleStart => {
                self.is_started = !self.is_started;
                self.time_start = Utc::now().timestamp_millis();
                true
            },
            Msg::SetDimensions(which, dimension) => {
                if which == 0 {
                    self.rows = dimension;
                } else {
                    self.cols = dimension;
                }
                true
            },
            Msg::SetPlacingMode(mode) => {
                self.place_mode = mode;
                true
            },
            Msg::SetStart(position) => {
                self.pos_start = position;
                true
            },
            Msg::SetTarget(position) => {
                self.pos_target = position;
                true
            },
            Msg::SetTime(time) => {
                self.time_end = time;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        let start = link.callback(|_| Msg::ToggleStart);

        let on_input_rows = link.callback(|e: Event| {
            Msg::SetDimensions(0, e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap())
        });

        let on_input_cols = link.callback(|e: Event| {
            Msg::SetDimensions(1, e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap())
        });

        let placemode = self.place_mode;

        let on_setstart = link.callback(|(x, y)| Msg::SetStart((x, y)));
        let on_settarget = link.callback(|(x, y)| Msg::SetTarget((x, y)));

        let on_solved = link.callback(|time| Msg::SetTime(time));

        html! {
            <div class="app">
                <div class="options">
                    <div class="field">
                        <label for="rows">{"Rows"}</label>
                        <input type="number" id="rows" value={self.rows.to_string()} onchange={on_input_rows} />
                    </div>
                    <div class="field">
                        <label for="cols">{"Columns"}</label>
                        <input type="number" id="cols" value={self.cols.to_string()} onchange={on_input_cols} />
                    </div>
                    <div class="field">
                        <label for="place-start">{"Place Start"}</label>
                        <input name="place" type="radio" id="place-start"
                            onchange={link.callback(|_| Msg::SetPlacingMode(PlaceMode::Start))}
                            checked={placemode == PlaceMode::Start}
                        />
                        <br />
                        <label for="place-target">{"Place Target"}</label>
                        <input name="place" type="radio" id="place-target"
                            onchange={link.callback(|_| Msg::SetPlacingMode(PlaceMode::Target))}
                            checked={placemode == PlaceMode::Target}
                        />
                        <br />
                        <label for="place-wall">{"Place Wall"}</label>
                        <input name="place" type="radio" id="place-wall"
                            onchange={link.callback(|_| Msg::SetPlacingMode(PlaceMode::Wall))}
                            checked={placemode == PlaceMode::Wall}
                        />
                    </div>
                    <button onclick={start}>{if self.is_started {"Stop"} else {"Start"}}</button>
                    <p>{self.time_end - self.time_start}</p>
                </div>
                <div class="grid-ph">
                    <Grid
                        dimensions={(self.rows, self.cols)}
                        start={self.pos_start}
                        target={self.pos_target}
                        is_started={self.is_started}
                        place_mode={placemode}
                        on_setstart={on_setstart}
                        on_settarget={on_settarget}
                        on_solved={on_solved}
                    />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}