use client::GameType;

use gloo_timers::callback::Timeout;
use yew::prelude::*;

pub struct Game {
    board: Vec<Vec<char>>,
    player1: String,
    player2: String,
    game_type: GameType,
    num_rows: i32,
    num_cols: i32,
    user_turn: bool,
    done: bool,
}

pub enum Msg {
    UserMove { col: usize },
    ComputerMove { col: usize },
    Reset,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub player1: String,
    pub player2: String,
    pub game_type: GameType,
    #[prop_or(7)]
    pub num_rows: i32,
    #[prop_or(6)]
    pub num_cols: i32,
}

impl Game {
    fn alpha_beta_minmax(
        &mut self,
        player: char,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, usize) {
        let max_player = match self.game_type {
            GameType::Connect4 => 'R',
            GameType::TootAndOtto => 'O',
        };

        let next_player = match player {
            'R' => 'B',
            'B' => 'R',
            'T' => 'O',
            'O' => 'T',
            _ => panic!("Invalid player!"),
        };

        if self.is_win() {
            if player == max_player {
                return (-1, 0); // Min player won
            } else {
                return (1, 0); // Max player won
            }
        } else if self.is_draw() || depth == 0 {
            return (0, 0);
        }

        let mut best_score = if player == max_player {
            i32::MIN
        } else {
            i32::MAX
        };
        let mut best_move = 0;

        for m in self.get_valid_moves() {
            self.perform_move(m, player);

            let (score, _) = self.alpha_beta_minmax(next_player, depth - 1, alpha, beta);

            if player == max_player {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }

                if beta <= best_score {
                    self.undo_move(m);
                    return (best_score, best_move);
                }
                alpha = std::cmp::max(alpha, best_score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
                if alpha >= best_score {
                    self.undo_move(m);
                    return (best_score, best_move);
                }
                beta = std::cmp::min(beta, best_score);
            }
            self.undo_move(m);
        }
        return (best_score, best_move);
    }

    fn is_draw(&self) -> bool {
        self.get_valid_moves().len() == 0
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

    fn undo_move(&mut self, col: usize) {
        for i in 0..self.num_rows {
            if self.board[i as usize][col as usize] != '_' {
                self.board[i as usize][col as usize] = '_';
                break;
            }
        }
    }

    fn perform_move(&mut self, col: usize, player: char) {
        for row in (0..self.num_rows).rev() {
            if self.board[row as usize][col as usize] == '_' {
                self.board[row as usize][col as usize] = player;
                break;
            }
        }
    }

    fn is_win(&self) -> bool {
        let sequences = match self.game_type {
            GameType::Connect4 => vec!["RRRR", "BBBB"],
            GameType::TootAndOtto => vec!["OTTO", "TOOT"],
        };

        for row in &self.board {
            let row_str: String = row.into_iter().collect();
            for sequence in sequences.clone() {
                if row_str.contains(&sequence) {
                    return true;
                }
            }
        }

        // Check columns for a winning sequence
        let n = self.board[0].len();
        for col_idx in 0..n {
            let col_str: String = self.board.iter().map(|row| row[col_idx]).collect();
            for sequence in sequences.clone() {
                if col_str.contains(&sequence) {
                    return true;
                }
            }
        }

        // Check all possible diagonals for a winning sequence
        let diagonals = get_diagonal_strings(&self.board);

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

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        let props = _ctx.props().clone();
        Self {
            board: vec![
                vec!['_'; props.num_cols.try_into().unwrap()];
                props.num_rows.try_into().unwrap()
            ],
            user_turn: true,
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
            Msg::Reset => {
                self.board = vec![
                    vec!['_'; self.num_cols.try_into().unwrap()];
                    self.num_rows.try_into().unwrap()
                ];
                self.done = false;

                true
            }
            Msg::UserMove { col } => {
                self.perform_move(
                    col,
                    match self.game_type {
                        GameType::Connect4 => 'R',
                        GameType::TootAndOtto => 'T',
                    },
                );

                if self.is_win() {
                    self.done = true;
                }

                self.user_turn = !self.user_turn;

                true
            }
            Msg::ComputerMove { col } => {
                self.perform_move(
                    col,
                    match self.game_type {
                        GameType::Connect4 => 'B',
                        GameType::TootAndOtto => 'O',
                    },
                );

                if self.is_win() {
                    self.done = true;
                }

                self.user_turn = !self.user_turn;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut board = vec![];

        for i in 0..self.num_cols {
            let onclick = link.callback(move |_| return Msg::UserMove { col: i as usize });

            let col: Vec<char> = self.board.iter().map(|row| row[i as usize]).collect();
            board.push(html! {
                <button disabled={self.done} class="column" onclick={onclick}>

                    { for col.iter().map(|item| html! {
                        match item {
                            'R' => html! { <div class="circle bounce" style="background-color: #ED5A8B;"></div> },
                            'B' => html! { <div class="circle bounce" style="background-color: #6F8FEA;text-align: center;"></div> },
                            'T'|'O' => html! { <div class="circle bounce" style="background-color: #FFFFFF;">{item}</div> },
                            _ => html! { <div class="circle"></div> },
                        }

                    })
                    }
                </button>
            });
        }
        let subtitle = if self.done {
            if self.user_turn {
                format!("{} wins!", self.player1)
            } else {
                format!("{} wins!", self.player2)
            }
        } else {
            if self.user_turn {
                format!("{}'s turn", self.player1)
            } else {
                format!("{}'s turn", self.player2)
            }
        };

        let title = match self.game_type {
            GameType::Connect4 => "Connect 4",
            GameType::TootAndOtto => "Toot & Otto",
        };

        html! {
            <div class="game-container">
            <h1 class="title">{ title }</h1>
            <h2 class="subtitle">{ subtitle }</h2>
            <div class="grid">{ board }</div>
            <button class="restart" onclick={link.callback(|_| Msg::Reset)}>{"Restart"}</button>
        </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if !self.user_turn && !self.done {
            let player = match self.game_type {
                GameType::Connect4 => 'B',
                GameType::TootAndOtto => 'O',
            };

            let (_, col) = self.alpha_beta_minmax(player, 5, i32::MIN, i32::MAX);

            let msg = Msg::ComputerMove { col };

            let link = ctx.link().clone();
            Timeout::new(500, move || {
                link.send_message(msg);
            })
            .forget();
        }
    }
}

#[function_component]
pub fn Connect4() -> Html {
    html! {
        <Game player1={"Ali".to_string()} player2={"AI - Medium".to_string()} game_type={GameType::Connect4}  />
    }
}
#[function_component]
pub fn TootOtto() -> Html {
    html! {
        <Game player1={"Rupin".to_string()} player2={"AI - Medium".to_string()} game_type={GameType::TootAndOtto} />
    }
}
