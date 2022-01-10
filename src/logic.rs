use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Game, Coord};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "gandhi56",
        "color": "#3c0c59",
        "head": "evil",
        "tail": "small-rattle",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

// ==================================================================

pub struct SearchState{
    board: Board,
    mysnake_id: String,
}

impl SearchState{
    fn new(board: Board, id: String) -> SearchState{
        SearchState{board: board, mysnake_id: id}
    }

    // TODO
    fn update(&self, mov: &str) /*SearchState*/{
        //let mut new_state = SearchState::new(self.board, self.mysnake_id);
        //let new_head_pos = next_pos();
    }

    // TODO
    fn get_snake_body(&self, id: String) -> Vec<Coord>{
        for snake in &self.board.snakes{
            if snake.id == id{
                return snake.body;
            }
        }
        return vec![];
    }
}

pub fn next_pos(pos: &Coord, mov: &str) -> Coord {
    match mov{
        "up" => Coord{x: pos.x, y: pos.y+1},
        "down" => Coord{x: pos.x, y: pos.y-1},
        "right" => Coord{x: pos.x+1, y: pos.y},
        _ => Coord{x: pos.x-1, y: pos.y},
    }
}

pub fn attacks_opponent(pos: &Coord, mysnake: &Battlesnake, board: &Board) -> bool {
    for snake in board.snakes.iter() {
        if snake.id == mysnake.id{
            continue;
        }
        if snake.body.contains(&pos) && snake.length >= mysnake.length {
            return true;
        }
    }
    false
}

pub fn filter_trivially_bad_moves<'a>(board: &Board,  you: &Battlesnake) -> HashMap<&'a str, bool>{
    
    // initialize possible moves
    let mut possible_moves: HashMap<&str, bool> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Do not let your Battlesnake move back in on its own neck
    let my_head = &you.head;
    let my_neck = &you.body[1];
    if my_neck.x < my_head.x {
        // my neck is left of my head
        possible_moves.insert("left", false);
    } else if my_neck.x > my_head.x {
        // my neck is right of my head
        possible_moves.insert("right", false);
    } else if my_neck.y < my_head.y {
        // my neck is below my head
        possible_moves.insert("down", false);
    } else if my_neck.y > my_head.y {
        // my neck is above my head
        possible_moves.insert("up", false);
    }

    let mut bad_moves: Vec<&str> = vec![];
    for (mov, _ok) in possible_moves.iter_mut(){
        let new_pos = next_pos(&you.head, mov);
        
        if  new_pos.x >= board.width || new_pos.y >= board.height || 
            you.body.contains(&new_pos) ||
            attacks_opponent(&new_pos, you, board){
            bad_moves.push(mov);
            continue;
        }
    }


    // If no possible move is available, then all moves
    // should be allowed. Otherwise, do not allow the
    // bad moves.
    if bad_moves.len() < 4{
        for mov in bad_moves{
            possible_moves.insert(mov, false);
        }
    }
    possible_moves
}

pub fn get_move(
    game: &Game,
    _turn: &u32,
    board: &Board,
    you: &Battlesnake) -> &'static str {

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.

    let moves = filter_trivially_bad_moves(board, you)
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}
