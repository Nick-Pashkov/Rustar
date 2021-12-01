use yew::prelude::*;
use weblog::console_log;

use crate::Position2D;
use crate::NodeState;
/*
fn calculate_h(position: Position2D, target: Position2D) -> i32 {
    let a = (target.0 - position.0).abs();
    let b = (target.1 - position.1).abs();
    a + b
}

fn calculate_g(position: Position2D, start: Position2D) -> i32 {
    (position.0 - start.0).abs() + (position.1 - start.1).abs()
}
*/

pub enum Msg {
    OnClick
}

fn create_default_state() -> NodeState {
    NodeState::None
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub x: i32,
    pub y: i32,
    #[prop_or_else(create_default_state)]
    pub state: NodeState,
    pub g: i32,
    pub h: i32,
    pub onclick: Callback<Position2D>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub state: NodeState,
    pub g: i32,
    pub h: i32,
    pub came_from: Option<Position2D>,
}

impl Node {
    pub fn get_f(&self) -> i32 {
        self.g + self.h
    }
}

impl Component for Node {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        let Props { x, y, state, .. } = ctx.props();

        Self {
            x: *x,
            y: *y,
            state: *state,
            g: 0,
            h: 0,
            came_from: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                ctx.props().onclick.emit((self.x, self.y));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let Props { state, g, h, x, y, .. } = *ctx.props();

        let mut status = "";
        if state == NodeState::Start { status = "start" }
        else if state == NodeState::Target { status = "target" }
        else if state == NodeState::Current { status = "current" }
        else if state == NodeState::Open { status = "open" }
        else if state == NodeState::Closed { status = "closed" }
        else if state == NodeState::Path { status = "path" }
        else if state == NodeState::Wall { status = "wall" }

        let classes = classes!["node", status];

        let onclick = ctx.link().callback(move |_| Msg::OnClick);

        html! {
            <div class={classes} onclick={onclick}>
                {if g != 0 {
                    html! {<span class="g">{g}</span>}
                }else {html!{}}}

                {if h != 0 {
                    html! {<span class="h">{h}</span>}
                }else {html!{}}}

                {if g != 0 && h != 0 {
                    html! {<span class="f">{g + h}</span>}
                }else {html!{}}}
            </div>
        }
    }
}