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

/// Manhattan distance
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
    list.iter().any(|&n| (n.x, n.y) == (node.x, node.y))
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
                neighbors.push(grid[checkX as usize][checkY as usize])
            }
        }
    }

    neighbors
}

pub fn algorithmV2(grid: &mut Vec<Vec<Node>>, target: Node, position: &mut Position2D, open_list: &mut Vec<Node>, closed_list: &mut Vec<Node>) -> bool {
    let mut current_node = open_list[0];
    *position = (current_node.x, current_node.y);
    console_log!(format!("Open List {:?}", open_list));

    for i in 1..open_list.len() {
        if open_list.get(i).is_none() {
            break;
        }
        if open_list[i].get_f() < current_node.get_f() || (open_list[i].get_f() == current_node.get_f() && open_list[i].h < current_node.h) {
            current_node = open_list[i];
            *position = (current_node.x, current_node.y);
        }
    }
    open_list.retain(|&r| (r.x, r.y) != (current_node.x, current_node.y));
    closed_list.push(current_node);
    grid[current_node.x as usize][current_node.y as usize].state = NodeState::Closed;

    if (current_node.x, current_node.y) == (target.x, target.y) {
        console_log!(format!("Path found {:?}", target));

        let mut trace: Vec<Position2D> = Vec::new();
        for item in closed_list.iter() {
            if let Some(pos) = item.came_from {
                if trace.contains(&pos) {
                    continue;
                }
                grid[pos.0 as usize][pos.1 as usize].state = NodeState::Path;
                trace.push(pos);
            }
        }
        
        console_log!(format!("Trace {:?}", trace));
        return true;
    }

    for mut neighbor in get_neighbors(grid, (current_node.x, current_node.y)) {
        if neighbor.state == NodeState::Wall || is_position_in_list(closed_list, &neighbor) {
            console_log!(format!("List {:?}", closed_list));
            console_log!(format!("Elem {:?}", neighbor));
            continue;
        }
        let new_move_cost = current_node.g + get_distance(&current_node, &neighbor);

        if new_move_cost < current_node.g || !open_list.contains(&neighbor) {
            neighbor.g = new_move_cost;
            neighbor.h = get_distance(&neighbor, &target);
            neighbor.came_from = Some((current_node.x, current_node.y));
            grid[neighbor.x as usize][neighbor.y as usize] = neighbor;

            if !open_list.contains(&neighbor) {
                open_list.push(neighbor.clone());
                grid[neighbor.x as usize][neighbor.y as usize].state = NodeState::Open;
                //return ((neighbor.x, neighbor.y), false)
            }
        }
    }
    false
}