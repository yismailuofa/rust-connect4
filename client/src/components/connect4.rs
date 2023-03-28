use std::cmp::min;
use log::info;
use wasm_bindgen::JsValue;
use yew::*;

use self::_Props::game_type;

pub struct Game {
    //link: ComponentLink<Self>,
    board: Vec<Vec<char>>,
    turn: bool,
    player1: String,
    player2: String,
    game_type: String,
    num_rows: i32,
    num_cols: i32,
    done: bool,
}

pub enum Msg {
    Move{
        col: i32,
    },
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub player1: String,
    pub player2: String,
    pub game_type: String,
    pub num_rows: i32,
    pub num_cols: i32,
}

// fn isWin(board: &Vec<Vec<char>>, row: i32, col: i32, player: char, g_type: String) -> bool {
//     wasm_logger::init(wasm_logger::Config::default());
//     let mut count = 0;
//     // check horizontal
//     let mut start = 0;
//     // go as far left as possible
//     loop {
//         start -= 1;
//         if (col+start)<0 ||board[row as usize][(col + start) as usize] != player {
//             start += 1;
//             break;
//         }
//     }
//     // check bounds
//     info!("start: {}", start);
//     if (board[0].len() - (col+start) as usize) > 3 {
//         count = 0;
//         for i in start..start+4 {
//             if board[row as usize][(col + i) as usize] == player {
//                 count += 1;
//             }
//         }
//         info!("count: {}", count);
//         if count == 4 {
//             return true;
//         }
//     }
    

//     // check vertical
//     // go as far up as possible
//     start = 0;
//     loop {
//         start -= 1;
//         if (row+start)<0 || board[(row + start) as usize][col as usize] != player {
//             start += 1;
//             break;
//         }
//     }
//     // check bounds
//     if (board.len() - (row+start) as usize) > 3 {
//         count = 0;
//         for i in start..start+4 {
//             if board[(row + i) as usize][col as usize] == player {
//                 count += 1;
//             }
//         }
//         if count == 4 {
//             return true;
//         }
//     }
//     // check diagonal
//     // go as far diagonal left up as possible
//     start = 0;
    
//     loop {
//         start -= 1;
//         if (row+start)<0 || (col+start)<0 || board[(row + start) as usize][(col + start) as usize] != player {
//             start += 1;
//             break;
//         }
//     }
//     count = 0;
//     // go diagonal right down as possible
//     if (board.len() - (row+start) as usize) > 3 && (board[0].len() - (col+start) as usize) > 3 {
//         for i in start..start+4 {
//             if board[(row + i) as usize][(col + i) as usize] == player {
//                 count += 1;
//             }
//         }
//         if count == 4 {
//             return true;
//         }
//     }
    

//     // check other diagonal
//     // go as far diagonal right up as possible
//     start = 0;
    
//     loop {
//         start -= 1;
//         if (row+start)<0 || (col-start)>=board[0].len() as i32 || board[(row + start) as usize][(col - start) as usize] != player {
//             start += 1;
//             break;
//         }
//     }
//     count = 0;
    
//     // go diagonal left down as possible
//     if (board.len() - (row+start) as usize) > 3 && (col+start) > 3 {
//         for i in start..start+4 {
//             if board[(row + i) as usize][(col - i) as usize] == player {
//                 count += 1;
//             }
//         }
//         if count == 4 {
//             return true;
//         }
//     }
//     false
// }

fn isWin(board: &Vec<Vec<char>>, sequences: Vec<&str>) -> bool {
    wasm_logger::init(wasm_logger::Config::default());
    // let sequence = format!("{}{}{}{}", player, player, player, player);

    for row in board {
        let row_str: String = row.into_iter().collect();
        for sequence in sequences.clone() {
            if row_str.contains(&sequence) {
                return true;
            }
        }
    }
    
    // Check columns for a winning sequence
    let n = board.len();
    for col_idx in 0..n {
        let col_str: String = board.iter().map(|row| row[col_idx]).collect();
        for sequence in sequences.clone() {
            if col_str.contains(&sequence) {
                return true;
            }
        }
    }
    
    // Check diagonals for a winning sequence
    let diag1: String = (0..n).map(|i| board[i][i]).collect();
    let diag2: String = (0..n).map(|i| board[i][n - 1 - i]).collect();
    
    for sequence in sequences {
        if diag1.contains(&sequence) || diag2.contains(&sequence) {
            return true;
        }
    }
    // if diag1.contains(&sequence) || diag2.contains(&sequence) {
    //     return true;
    // }
    // if diag1.contains() || diag2.contains(&sequence) {
    //     return "OTTO";
    // }
    
    // No winning sequence found
    false
}





impl Component for Game {
    type Message = Msg;
    type Properties = (Props);
    fn create(_ctx: &Context<Self>) -> Self {
        let props = _ctx.props().clone();
        Self {
            //link,
            board: vec![vec!['_'; props.num_cols.try_into().unwrap()]; props.num_rows.try_into().unwrap()],
            turn: true,
            player1: props.player1.clone(),
            player2: props.player2.clone(),
            game_type: props.game_type.clone(),
            num_rows: props.num_rows,
            num_cols: props.num_cols,
            done: false,
        }
    }
    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Move{col} => {
                let mut row = 0;
                for i in 0..self.num_rows {
                    if self.board[i as usize][col as usize] == '_' {
                        row = i;
                        //break;
                    }
                }
                let mut sequences = Vec::new();
                match self.game_type.as_str() {
                    "connect4" => {
                        if self.board[row as usize][col as usize] != '_' {
                            return false;
                        }
                        if self.turn {
                            self.board[row as usize][col as usize] = 'R';
                            //add to vector of 4 R's in a row
                            sequences.push("RRRR");

                        } else {
                            self.board[row as usize][col as usize] = 'Y';
                            //sequence is 4 Y's in a row
                            sequences.push("YYYY");
                        }
                    }
                    "otto" => {
                        if self.board[row as usize][col as usize] != '_' {
                            return false;
                        }
                        if self.turn {
                            self.board[row as usize][col as usize] = 'O';
                        } else {
                            self.board[row as usize][col as usize] = 'T';
                        }
                        sequences.push("OTTO");
                        sequences.push("TOOT");
                    }
                    _ => {}
                }
                    
                
                if isWin(&self.board, sequences) {
                    println!("{} wins!", if self.turn {&self.player2} else {&self.player1});
                    self.done = true;
                }
                self.turn = !self.turn;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut board = vec![];
        for i in 0..self.num_rows {
            let mut row = vec![];
            for j in 0..self.num_cols {
                let onclick = link.callback(move |_| Msg::Move{col: j});
                row.push(html! {
                    <button onclick={onclick}>{ self.board[i as usize][j as usize] }</button>
                });
            }
            board.push(html! {
                <div>{ row }</div>
            });
        }
        html! {
            <div>
                <h1>{ "Connect 4" }</h1>
                <h2>{ "Player 1: " }{ &self.player1 }</h2>
                <h2>{ "Player 2: " }{ &self.player2 }</h2>
                <h2>{ "Turn: " }{ if self.turn { &self.player1 } else { &self.player2 } }</h2>
                <h2>{ "Done? : " }{ &self.done }</h2>
                <div>{ board }</div>
            </div>
        }
    }

    

}

#[function_component]
pub fn Connect4() -> Html {
    html! {
        <Game player1={"Muneer".to_string()} player2={"Ismail".to_string()} game_type={"Connect 4".to_string()} num_rows={6} num_cols={6} />
    }
}