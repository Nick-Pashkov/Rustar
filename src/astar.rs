use crate::node::Node;
use crate::Position2D;
use crate::NodeState;

use weblog::{console_log, console_time, console_time_log};

#[derive(Debug)]
struct Costs {
    h: i32,
    g: i32,
    f: i32,
    location: Position2D
}

fn calculate_h(position: Position2D, target: Position2D) -> i32 {
    let a = (target.0 - position.0).abs();
    let b = (target.1 - position.1).abs();
    a + b
}

fn calculate_g(position: Position2D, start: Position2D) -> i32 {
    (position.0 - start.0).abs() + (position.1 - start.1).abs()
}

/// Euclidean distance
fn get_distance(nodeA: &Node, nodeB: &Node) -> i32 {
    let dstX = (nodeA.x - nodeB.x).abs();
    let dstY = (nodeA.y - nodeB.y).abs();

    if dstX > dstY {
        return 14*dstY + 10*(dstX-dstY);
    } else {
        return 14*dstX + 10*(dstY-dstX);
    }
}

fn is_position_in_list(list: &mut Vec<Node>, node: &Node) -> bool {
    list.iter().any(|n| (n.x, n.y) == (node.x, node.y))
}

fn print_list(list: &mut Vec<Node>) {
    let mut result = String::new();
    for elem in list {
        result.push_str(&format!("{}-{} {}\n", elem.x, elem.y, elem.get_f()))
    }
    console_log!(result);
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

            let checkX = x + neighbor_x;
            let checkY = y + neighbor_y;

            if (checkX > -1 && checkX < grid_width) && (checkY > -1 && checkY < grid_height) {
                neighbors.push(grid[checkX as usize][checkY as usize].clone())
            }
        }
    }

    neighbors
}

pub fn algorithmV2(grid: &mut Vec<Vec<Node>>, target: Node, position: &mut Position2D, open_list: &mut Vec<Node>, closed_list: &mut Vec<Node>) -> Option<Vec<Node>> {
    let mut current_node = open_list[0].clone();
    *position = (current_node.x, current_node.y);

    for i in 1..open_list.len() {
        if open_list[i].get_f() < current_node.get_f() || (open_list[i].get_f() == current_node.get_f() && open_list[i].h < current_node.h) {
            current_node = open_list[i].clone();
            *position = (current_node.x, current_node.y);
        }
    }
    open_list.retain(|r| (r.x, r.y) != (current_node.x, current_node.y));
    closed_list.push(current_node.clone());
    if current_node.state != NodeState::Start || current_node.state != NodeState::Target {
        grid[current_node.x as usize][current_node.y as usize].state = NodeState::Closed;
    }

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
        if neighbor.state == NodeState::Wall || is_position_in_list(closed_list, &neighbor) {
            continue;
        }
        let n_x = neighbor.x;
        let n_y = neighbor.y;
        let in_open_list = is_position_in_list(open_list, &neighbor);

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
                open_list.push(neighbor.clone());
            } else {
                if let Some(pos) = open_list.iter().position(|n| (n.x, n.y) == (n_x, n_y)) {
                    open_list[pos].g = neighbor.g;
                    open_list[pos].h = neighbor.h;
                    open_list[pos].parent = neighbor.parent.clone();
                }
            }
        }
        grid[n_x as usize][n_y as usize] = neighbor;
    }
    None
}