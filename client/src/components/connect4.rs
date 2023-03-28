use std::i32::{MIN, MAX};

use log::info;
use yew::*;

pub struct Game {
    board: Vec<Vec<char>>,
    turn: bool,
    player1: String,
    player2: String,
    game_type: String,
    num_rows: i32,
    num_cols: i32,
    done: bool,
    //grid:
}

pub enum Msg {
    Move { col: usize },
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub player1: String,
    pub player2: String,
    pub game_type: String,
    pub num_rows: i32,
    pub num_cols: i32,
}

impl Game {
    fn alpha_beta_minmax(&mut self, player:bool, depth: i32, mut alpha: i32, mut beta: i32) -> (i32, usize) {
        // make next_player an empty hashmap
        let maxPlayer = true;
        let sequences = match self.game_type.as_str() {
            "connect4" => if player {vec!["RRRR"]} else {vec!["OOOO"]},
            "otto" => vec!["OTTO", "TOOT"],
            _ => vec!["XXXX"],
        };
            
        if is_win(&self.board, sequences) {
            if player {
                return (1, 0);
            } else {
                return (-1, 0);
            }
        }
        else if self.is_draw() || depth == 0 {
            return (0, 0);
        }
        let mut bestScore = if maxPlayer == player {i32::MIN} else {i32::MAX};
        let mut bestMove = 0;
        for m in self.get_valid_moves() {
            let row = self.get_first_empty_row(m);
            self.board[row as usize][m] = if player {'R'} else {'Y'};
            let score = self.alpha_beta_minmax(!player, depth - 1, alpha, beta).0;
            
            if maxPlayer == player {
                if score > bestScore {
                    bestScore = score;
                    bestMove = m;
                }
                if beta <= bestScore {
                    let row = self.get_first_empty_row(m) +1;
                    self.board[row as usize][m] = '_';
                    return (bestScore, bestMove);
                }
                alpha = if alpha > bestScore {alpha} else {bestScore};

            } else {
                if score < bestScore {
                    bestScore = score;
                    bestMove = m;
                }
                if alpha >= bestScore {
                    let row = self.get_first_empty_row(m)+1;
                    self.board[row as usize][m] = '_';
                    return (bestScore, bestMove);
                }
                beta = if beta < bestScore {beta} else {bestScore};

            }
            let row = self.get_first_empty_row(m)+1;
            self.board[row as usize][m] = '_';
            //self.board[(self.get_first_empty_row(m )+1) as usize][m] = '_';
        }
        return (bestScore, bestMove);
        
    }
    fn get_first_empty_row(&self, col: usize) -> i32 {
        let mut row = -1;
        for i in 0..self.num_rows {
            if self.board[i as usize][col as usize] == '_' {
                row = i;
                //break;
            }
        }
        row
    }
    fn is_draw(&self) -> bool {
        for i in 0..self.num_cols {
            if self.board[0][i as usize] == '_' {
                return false;
            }
        }
        true
    }
    fn get_valid_moves(&self) -> Vec<usize> {
        let mut moves = vec![];
        for i in 0..self.num_cols {
            if self.board[0][i as usize] == '_' {
                moves.push(i as usize);
            }
        }
        moves
    }
    
    
}

fn get_diagonal_strings(matrix: &[Vec<char>]) -> Vec<String> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Define a closure to extract diagonals given a starting point (r, c) and step values.
    let extract_diag = |r: isize, c: isize, step_r: isize, step_c: isize| -> String {
        (0..)
            .map(move |i| {
                let row = r + i * step_r;
                let col = c + i * step_c;
                (row, col)
            })
            .take_while(|&(row, col)| {
                row >= 0 && row < rows as isize && col >= 0 && col < cols as isize
            })
            .map(|(row, col)| matrix[row as usize][col as usize])
            .collect()
    };

    let mut diagonals = vec![];

    // Extract primary diagonals starting from each element of the top row (going down-right).
    diagonals.extend((0..cols).map(|col| extract_diag(0, col as isize, 1, 1)));

    // Extract primary diagonals starting from each element of the left column (going down-right), excluding top-left corner.
    diagonals.extend((1..rows).map(|row| extract_diag(row as isize, 0, 1, 1)));

    // Extract secondary diagonals starting from each element of the top row (going down-left).
    diagonals.extend((0..cols).map(|col| extract_diag(0, col as isize, 1, -1)));

    // Extract secondary diagonals starting from each element of the right column (going down-left), excluding top-right corner.
    diagonals.extend((1..rows).map(|row| extract_diag(row as isize, cols as isize - 1, 1, -1)));

    diagonals
}

fn is_win(board: &Vec<Vec<char>>, sequences: Vec<&str>) -> bool {
    wasm_logger::init(wasm_logger::Config::default());

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

    // Check all possible diagonals for a winning sequence
    let diagonals = get_diagonal_strings(&board);
    info!("Diagonals: {:?}", diagonals);
    for diagonal in diagonals {
        for sequence in sequences.clone() {
            if diagonal.contains(&sequence) {
                return true;
            }
        }
    }
    // No winning sequence found
    false
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        
        let props = _ctx.props().clone();
        Self {
            //link,
            board: vec![
                vec!['_'; props.num_cols.try_into().unwrap()];
                props.num_rows.try_into().unwrap()
            ],
            turn: true,
            player1: props.player1.clone(),
            player2: props.player2.clone(),
            game_type: props.game_type.clone(),
            num_rows: props.num_rows,
            num_cols: props.num_cols,
            done: false,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("Message");
        match msg {
            
            Msg::Move { col } => {
                // let mut row = 0;
                // for i in 0..self.num_rows {
                //     if self.board[i as usize][col as usize] == '_' {
                //         row = i;
                //         //break;
                //     }
                // }
                let row = self.get_first_empty_row(col);
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

                if is_win(&self.board, sequences) {
                    println!(
                        "{} wins!",
                        if self.turn {
                            &self.player2
                        } else {
                            &self.player1
                        }
                    );
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
        // for i in 0..self.num_rows {
        //     let mut row = vec![];
        //     for j in 0..self.num_cols {
        //         let onclick = link.callback(move |_| Msg::Move { col: j });
        //         row.push(html! {
        //             <button onclick={onclick}>{ self.board[i as usize][j as usize] }</button>
        //         });
        //     }
        //     board.push(html! {
        //         <div>{ row }</div>
        //     });
        // }
        for i in 0..self.num_cols {
            let onclick = link.callback(move |_| Msg::Move { col: i as usize });
            let col: Vec<char> = self.board.iter().map(|row| row[i as usize]).collect();
            board.push(html! {
                <button class="column" onclick={onclick}>
                
                    { for col.iter().map(|item| html! { 
                        match item {
                            'R' => html! { <div class="circle bounce" style="background-color: #ED5A8B;"></div> },
                            'Y' => html! { <div class="circle bounce" style="background-color: #6F8FEA;text-align: center;"></div> },
                            'T'|'O' => html! { <div class="circle" style="background-color: #FFFFFF;">{item}</div> },
                            _ => html! { <div class="circle"></div> },
                        }
                    }) }

                
                </button>
            });
        }
        html! {
            <div>
                <h1>{ "Connect 4" }</h1>
                <h2>{ "Player 1: " }{ &self.player1 }</h2>
                <h2>{ "Player 2: " }{ &self.player2 }</h2>
                <h2>{ "Turn: " }{ if self.turn { &self.player1 } else { &self.player2 } }</h2>
                <h2>{ "Done? : " }{ &self.done }</h2>
                <div class="grid">{ board }</div>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !self.turn && !first_render{
            let col = self.alpha_beta_minmax(self.turn, 4, MIN, MAX).1;
            let msg = Msg::Move { col };
            ctx.link().send_message(msg);
        }
    }
}

#[function_component]
pub fn Connect4() -> Html {
    html! {
        <Game player1={"Muneer".to_string()} player2={"Ismail".to_string()} game_type={"connect4".to_string()} num_rows={6} num_cols={6} />
    }
}
#[function_component]
pub fn TootOtto() -> Html {
    html! {
        <Game player1={"Muneer".to_string()} player2={"Ismail".to_string()} game_type={"otto".to_string()} num_rows={6} num_cols={6} />
    }
}
