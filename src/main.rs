use yew::prelude::*;
use weblog::console_log;
use gloo::timers::callback::Interval;

mod node;
use node::Node;

mod astar;

pub type Position2D = (i32, i32);

enum Msg {
    Tick,
    CreateWall(Position2D),
    ToggleStart
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

struct Model {
    dimensions: Position2D,
    start: Node,
    target: Node,
    grid: Vec<Vec<Node>>,
    pos: Position2D,
    interval: Interval,
    open_list: Vec<Node>,
    closed_list: Vec<Node>,
    is_solved: bool,
    is_started: bool,
}

impl Model {
    fn render_grid(&self, ctx: &Context<Self>) -> Html {
        let grid = &self.grid;

        let mut nodes: Vec<Html> = Vec::new();

        for x in 0..(self.dimensions.0) {
            for y in 0..self.dimensions.1 {
                let cur_node = &grid[x as usize][y as usize];
                let create_wall = ctx.link().callback(|(x, y)| Msg::CreateWall((x, y)));
                nodes.push(html!{
                    
                    <Node
                        key={x+1*y+1}
                        x={x}
                        y={y}
                        state={cur_node.state.clone()}
                        h={cur_node.h}
                        g={cur_node.g}
                        onclick={create_wall}
                    />
                })
            }
        }

        nodes.into_iter().collect::<Html>()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let num_rows = 20;
        let num_cols = 16;
        let start_position: Position2D = (1, 3);
        let target_position: Position2D = (17, 1);

        let mut grid: Vec<Vec<Node>> = Vec::new();
        for rows in 0..num_rows {
            let mut vec_rows: Vec<Node> = Vec::new();
            for cols in 0..num_cols {

                let state = if rows == start_position.0 && cols == start_position.1 { NodeState::Start }
                    else if rows == target_position.0 && cols == target_position.1 { NodeState::Target }
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

        let interval = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::Tick))
        };

        let start = grid[start_position.0 as usize][start_position.1 as usize].clone();
        let target = grid[target_position.0 as usize][target_position.1 as usize].clone();
        // Create open List
        let mut open_list: Vec<Node> = Vec::new();
        // Create closed List
        let closed_list: Vec<Node> = Vec::new();

        // Add start to open list
        open_list.push(start.clone());

        let position = (start.x, start.y);

        Self {
            dimensions: (num_rows, num_cols),
            start,
            target,
            grid,
            pos: position,
            interval,
            open_list,
            closed_list,
            is_solved: false,
            is_started: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                //console_log!(format!("Open List before {:?}", open_list));
                if !self.is_solved && self.is_started {
                    self.is_solved = astar::algorithmV2(&mut self.grid, self.target.clone(), &mut self.pos, &mut self.open_list, &mut self.closed_list).is_some();
                    return true;
                }
                //console_log!(format!("Open List after {:?}", open_list));
                false
            },
            Msg::CreateWall(position) => {
                let mut node = &mut self.grid[position.0 as usize][position.1 as usize];
                if node.state == NodeState::Wall {
                    node.state = NodeState::None;
                } else {
                    node.state = NodeState::Wall;
                }
                true
            },
            Msg::ToggleStart => {
                self.is_started = !self.is_started;
                true
            }
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        //ctx.link().send_message(Msg::Tick)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        //let start = ctx.link().callback(|_| Msg::ToggleStart);
        //let step = ctx.link().callback(|_| Msg::Tick);

        html! {
            <>
                //<button onclick={start}>{if self.is_started {"Stop"} else {"Start"}}</button>
                //<button onclick={step}>{{"+1"}}</button>
                <div class="grid" style={format!("grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr);", self.dimensions.0, self.dimensions.1)}>
                    { self.render_grid(ctx) }
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}