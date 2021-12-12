use yew::prelude::*;
use weblog::console_log;
use chrono::Utc;
use gloo::timers::callback::Interval;

use crate::{Position2D, Node, NodeState, astar::Astar, PlaceMode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub dimensions: Position2D,
    pub start: Position2D,
    pub target: Position2D,
    pub is_started: bool,
    pub place_mode: PlaceMode,
    pub on_setstart: Callback<Position2D>,
    pub on_settarget: Callback<Position2D>,
    pub on_solved: Callback<i64>,
}

pub enum Msg {
    PlaceCell(Position2D),
    Step,
    SetNoneAt(Position2D),
    Start,
}

pub struct Grid {
    pub grid: Vec<Vec<Node>>,
    pub astar: Astar,
    pub target: Node,
    pub position: Position2D,
    pub is_solved: bool,
    pub task: Option<Interval>,
    pub flag: bool,
}

fn create_grid(dimensions: Position2D, start: Position2D, target: Position2D) -> Vec<Vec<Node>> {
    let mut grid: Vec<Vec<Node>> = Vec::new();
    for rows in 0..dimensions.0 {
        let mut vec_rows: Vec<Node> = Vec::new();
        for cols in 0..dimensions.1 {

            let state = if rows == start.0 && cols == start.1 { NodeState::Start }
                else if rows == target.0 && cols == target.1 { NodeState::Target }
                else { NodeState::None };

            vec_rows.push(Node {
                x: rows,
                y: cols,
                state,
                h: 0,
                g: 0,
                came_from: None,
                parent: None,
            });
        }
        grid.push(vec_rows);
    }
    grid
}

impl Grid {
    fn render_grid(&self, ctx: &Context<Self>) -> Html {
        let Props { dimensions, .. } = ctx.props();

        let mut nodes: Vec<Html> = Vec::new();
        for x in 0..(dimensions.0) {
            for y in 0..(dimensions.1) {
                let cur_node = &self.grid[x as usize][y as usize];
                let link = ctx.link().clone();
                nodes.push(html!{
                    <Node
                        x={x}
                        y={y}
                        state={cur_node.state.clone()}
                        h={cur_node.h}
                        g={cur_node.g}
                        onclick={link.callback(move |_| Msg::PlaceCell((x, y)))}
                    />
                })
            }
        }

        nodes.into_iter().collect::<Html>()
    }
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        let Props { dimensions, start, target, .. } = *ctx.props();

        let grid = create_grid(dimensions, start, target);

        let start = grid[start.0 as usize][start.1 as usize].clone();
        let target = grid[target.0 as usize][target.1 as usize].clone();

        let position = (start.x, start.y);

        let mut astar = Astar::new();
        astar.init(start, target.clone());

        Self {
            grid,
            astar,
            target,
            position,
            is_solved: false,
            task: None,
            flag: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let Props { dimensions, start, target, is_started, .. } = *ctx.props();

        if is_started { return true }

        let old_rows: i32 = self.grid.len().try_into().unwrap();
        let old_cols: i32 = self.grid[0].len().try_into().unwrap();

        if dimensions.0 != old_rows || dimensions.1 != old_cols {
            self.grid = create_grid(dimensions, start, target);
            return true
        }
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Props { place_mode, on_setstart, on_settarget, on_solved, start, target, .. } = ctx.props();
        match msg {
            Msg::PlaceCell(position) => {
                let mut node = &mut self.grid[position.0 as usize][position.1 as usize];
                match place_mode {
                    PlaceMode::Wall => {
                        if node.state == NodeState::Wall { node.state = NodeState::None } else { node.state = NodeState::Wall }
                    }
                    PlaceMode::Start => {
                        ctx.link().send_message(Msg::SetNoneAt(*start));
                        node.state = NodeState::Start;
                        //let start = self.grid[node.x as usize][node.y as usize].clone();
                        //let target = grid[target.0 as usize][target.1 as usize].clone();
                        self.astar.set_start(node.clone());
                        on_setstart.emit((node.x, node.y))
                    }
                    PlaceMode::Target => {
                        ctx.link().send_message(Msg::SetNoneAt(*target));
                        node.state = NodeState::Target;

                        self.astar.set_target(node.clone());
                        on_settarget.emit((node.x, node.y))
                    }
                    _ => {
                        console_log!("Other")
                    }
                }
                true
            },
            Msg::Step => {
                if self.is_solved {
                    return true
                }

                if self.astar.algorithm(&mut self.grid, &mut self.position).is_some() {
                    self.is_solved = true;
                    on_solved.emit(Utc::now().timestamp_millis());
                }
                true
            },
            Msg::SetNoneAt(position) => {
                let mut node = &mut self.grid[position.0 as usize][position.1 as usize];
                node.state = NodeState::None;
                true
            },
            Msg::Start => {
                self.flag = true;
                let task = {
                    let link = ctx.link().clone();
                    Interval::new(50, move || {
                        link.send_message(Msg::Step);
                    })
                };
                self.task = Some(task);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let Props { dimensions, is_started, .. } = *ctx.props();

        if !self.flag && is_started && !self.is_solved {
            ctx.link().send_message(Msg::Start);
        }

        html! {
            <div class="grid" style={format!("grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr);", dimensions.0, dimensions.1)}>
                { self.render_grid(ctx) }
            </div>
        }
    }
}