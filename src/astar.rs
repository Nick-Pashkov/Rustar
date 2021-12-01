use crate::node::Node;
use crate::Position2D;
use crate::NodeState;

use weblog::console_log;

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
    (nodeB.x - nodeA.x).abs() + (nodeB.y - nodeA.y).abs()
}

fn get_neighbors(grid: Vec<Vec<Node>>, position: Position2D) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let x = position.0 as usize;
    let y = position.1 as usize;

    if x > 0 && grid[x-1][y].state != NodeState::Closed {
        neighbors.push(grid[x-1][y].clone());
    }

    if x < grid.len() - 1 && grid[x+1][y].state != NodeState::Closed {
        neighbors.push(grid[x+1][y].clone());
    }

    if y > 0 && grid[x][y-1].state != NodeState::Closed {
        neighbors.push(grid[x][y-1].clone());
    }

    if y < grid[grid.len() - 1].len() - 1 && grid[x][y+1].state != NodeState::Closed {
        neighbors.push(grid[x][y+1].clone());
    }

    neighbors
}

pub fn algorithmV2(grid: &mut Vec<Vec<Node>>, target: Node, position: &mut Position2D, open_list: &mut Vec<Node>, closed_list: &mut Vec<Node>) -> bool {
    let mut current_node = open_list[0].clone();
    *position = (current_node.x, current_node.y);
    //console_log!(format!("Open List {:?}", open_list));

    for i in 1..open_list.len() {
        if open_list.get(i).is_none() {
            break;
        }
        if open_list[i].get_f() < current_node.get_f() || (open_list[i].get_f() == current_node.get_f() && open_list[i].h < current_node.h) {
            current_node = open_list[i].clone();
            *position = (current_node.x, current_node.y);
        }
        open_list.remove(open_list.iter().position(|&r| r == current_node).unwrap_or_default());
        closed_list.push(current_node);
        grid[current_node.x as usize][current_node.y as usize].state = NodeState::Closed;
    }

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

    for mut neighbor in get_neighbors(grid.to_vec(), (current_node.x, current_node.y)) {
        if neighbor.state == NodeState::Wall || closed_list.contains(&&neighbor) {
            continue;
        }
        let new_move_cost = current_node.g + get_distance(&current_node, &neighbor);

        if new_move_cost < current_node.g || !open_list.contains(&&neighbor) {
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