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
    Move { col: i32 },
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub player1: String,
    pub player2: String,
    pub game_type: String,
    pub num_rows: i32,
    pub num_cols: i32,
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
            .take_while(|&(row, col)| row >= 0 && row < rows as isize && col >= 0 && col < cols as isize)
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
        _ctx.link().send_message(Msg::Move { col: 0 });
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
        match msg {
            Msg::Move { col } => {
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
        for i in 0..self.num_rows {
            let mut row = vec![];
            for j in 0..self.num_cols {
                let onclick = link.callback(move |_| Msg::Move { col: j });
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
        <Game player1={"Muneer".to_string()} player2={"Ismail".to_string()} game_type={"connect4".to_string()} num_rows={6} num_cols={6} />
    }
}
#[function_component]
pub fn TootOtto() -> Html {
    html! {
        <Game player1={"Muneer".to_string()} player2={"Ismail".to_string()} game_type={"otto".to_string()} num_rows={6} num_cols={6} />
    }
}
