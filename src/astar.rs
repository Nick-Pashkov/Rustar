use crate::node::Node;
use crate::Position2D;
use crate::NodeState;

#[derive(Debug)]
struct Costs {
    h: i32,
    g: i32,
    f: i32,
    location: Position2D
}

#[derive(Debug)]
pub struct Astar {
    open_list: Vec<Node>,
    closed_list: Vec<Node>,
    target: Option<Node>,
    start: Option<Node>,
}

impl Astar {
    pub fn new() -> Self {
        // Create open List
        let open_list: Vec<Node> = Vec::new();
        // Create closed List
        let closed_list: Vec<Node> = Vec::new();

        Self {
            open_list,
            closed_list,
            target: None,
            start: None,
        }
    }

    pub fn init(&mut self, start: Node, target: Node) {

        self.open_list.clear();
        self.closed_list.clear();

        self.start = Some(start.clone());

        self.open_list.push(start);
        self.target = Some(target);
    }

    pub fn set_start(&mut self, start: Node) {
        self.open_list.clear();
        self.closed_list.clear();

        self.start = Some(start.clone());
        self.open_list.push(start);
    }

    pub fn set_target(&mut self, target: Node) {
        self.open_list.clear();
        self.closed_list.clear();

        self.open_list.push(self.start.clone().unwrap());
        self.target = Some(target);
    }

    pub fn algorithm(&mut self, grid: &mut Vec<Vec<Node>>, position: &mut Position2D) -> Option<Vec<Node>> {
        let mut current_node = self.open_list[0].clone();
        *position = (current_node.x, current_node.y);
    
        for i in 1..self.open_list.len() {
            if self.open_list[i].get_f() < current_node.get_f() || (self.open_list[i].get_f() == current_node.get_f() && self.open_list[i].h < current_node.h) {
                current_node = self.open_list[i].clone();
                *position = (current_node.x, current_node.y);
            }
        }
        self.open_list.retain(|r| (r.x, r.y) != (current_node.x, current_node.y));
        self.closed_list.push(current_node.clone());
        if current_node.state != NodeState::Start || current_node.state != NodeState::Target {
            grid[current_node.x as usize][current_node.y as usize].state = NodeState::Closed;
        }

        let target = self.target.as_ref().unwrap();
    
        if (current_node.x, current_node.y) == (target.x, target.y) {
            let mut path: Vec<Node> = Vec::new();
            loop {
                match current_node.parent {
                    Some(p) => {
                        current_node = *p;
                        path.push(current_node.clone());
                        grid[current_node.x as usize][current_node.y as usize].state = NodeState::Path;
                    },
                    None => break
                }
            }
            path.reverse();
            path.push(target.clone());
            grid[target.x as usize][target.y as usize].state = NodeState::Path;
            return Some(path);
        }
    
        for mut neighbor in get_neighbors(grid, (current_node.x, current_node.y)) {
            if neighbor.state == NodeState::Wall || is_position_in_list(&mut self.closed_list, &neighbor) {
                continue;
            }
            let n_x = neighbor.x;
            let n_y = neighbor.y;
            let in_open_list = is_position_in_list(&mut self.open_list, &neighbor);
    
            let new_move_cost = current_node.g + get_distance(&current_node, &neighbor);
    
            if new_move_cost < neighbor.g || !in_open_list {
                neighbor.g = new_move_cost;
                neighbor.parent = Some(Box::new(current_node.clone()));
                neighbor.h = get_distance(&neighbor, &target);
                neighbor.state = NodeState::Open;
                grid[n_x as usize][n_y as usize].g = new_move_cost;
                grid[n_x as usize][n_y as usize].h = neighbor.h;
    
                if !in_open_list {
                    grid[n_x as usize][n_y as usize].state = NodeState::Open;
                    self.open_list.push(neighbor.clone());
                } else {
                    if let Some(pos) = self.open_list.iter().position(|n| (n.x, n.y) == (n_x, n_y)) {
                        self.open_list[pos].g = neighbor.g;
                        self.open_list[pos].h = neighbor.h;
                        self.open_list[pos].parent = neighbor.parent.clone();
                    }
                }
            }
            grid[n_x as usize][n_y as usize] = neighbor;
        }
        None
    }
}

/// Euclidean distance
fn get_distance(node_a: &Node, node_b: &Node) -> i32 {
    let dst_x = (node_a.x - node_b.x).abs();
    let dst_y = (node_a.y - node_b.y).abs();

    if dst_x > dst_y {
        return 14*dst_y + 10*(dst_x-dst_y);
    } else {
        return 14*dst_x + 10*(dst_y-dst_x);
    }
}

fn is_position_in_list(list: &mut Vec<Node>, node: &Node) -> bool {
    list.iter().any(|n| (n.x, n.y) == (node.x, node.y))
}

fn get_neighbors(grid: &mut Vec<Vec<Node>>, position: Position2D) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let x = position.0;
    let y = position.1;

    let grid_width: i32 = grid.len().try_into().unwrap();
    let grid_height: i32 = grid[0].len().try_into().unwrap();

    for neighbor_x in -1..2 {
        for neighbor_y in -1..2 {
            if neighbor_x == 0 && neighbor_y == 0 {
                continue;
            }

            let check_x = x + neighbor_x;
            let check_y = y + neighbor_y;

            if (check_x > -1 && check_x < grid_width) && (check_y > -1 && check_y < grid_height) {
                neighbors.push(grid[check_x as usize][check_y as usize].clone())
            }
        }
    }

    neighbors
}