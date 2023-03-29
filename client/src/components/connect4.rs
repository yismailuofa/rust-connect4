use client::GameType;

use gloo_timers::callback::Timeout;
use js_sys::Math;
use log::info;
use yew::prelude::*;

pub struct Game {
    board: Vec<Vec<char>>,
    player1: String,
    player2: String,
    game_type: GameType,
    num_rows: i32,
    num_cols: i32,
    user_turn: bool,
    winners: (bool, bool),
    user_otto_toot: String,
    t_selected: bool,
}

pub enum Msg {
    UserMove { col: usize, choice: char },
    ComputerMove { col: usize, choice: char },
    Reset,
    RenderAgain,
    ChangeSel,
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
        player: bool,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, usize, char) {
        let max_player = true; //true is user move (R in connect4)

        // let next_player = match player {
        //     'R' => 'B',
        //     'B' => 'R',
        //     'T' => 'O',
        //     'O' => 'T',
        //     _ => panic!("Invalid player!"),
        // };
        let options = match self.game_type {
            GameType::Connect4 => {
                if player {
                    "R"
                } else {
                    "B"
                }
            }
            GameType::TootAndOtto => "OT",
        };

        // first element in winners array is user win, second is computer win
        let winners = self.is_win();
        if (winners.0 || winners.1) && (winners.0 != winners.1) {
            if winners.1 {
                return (-1, 0, '_'); // Min player won (computer)
            } else {
                return (1, 0, '_'); // Max player won
            }
        } else if self.is_draw() || depth == 0 || (winners.0 && winners.1) {
            return (0, 0, '_');
        }

        let mut best_score = if player == max_player {
            i32::MIN
        } else {
            i32::MAX
        };
        let mut best_move = 0;
        let mut best_choice = '_';
        for m in self.get_valid_moves() {
            for c in options.chars() {
                self.perform_move(m, c);

                let (score, _, _) = self.alpha_beta_minmax(!player, depth - 1, alpha, beta);

                if player == max_player {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                        best_choice = c;
                    }

                    if beta <= best_score {
                        self.undo_move(m);
                        return (best_score, best_move, best_choice);
                    }
                    alpha = std::cmp::max(alpha, best_score);
                } else {
                    if score < best_score {
                        best_score = score;
                        best_move = m;
                        best_choice = c;
                    }
                    if alpha >= best_score {
                        self.undo_move(m);
                        return (best_score, best_move, best_choice);
                    }
                    beta = std::cmp::min(beta, best_score);
                }
                self.undo_move(m);
            }
        }
        return (best_score, best_move, best_choice);
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

    fn is_win(&self) -> (bool, bool) {
        let sequences = match self.game_type {
            GameType::Connect4 => vec!["RRRR", "BBBB"],
            GameType::TootAndOtto => vec!["OTTO", "TOOT"],
        };
        let mut res = [false, false];

        for row in &self.board {
            let row_str: String = row.into_iter().collect();
            for (i, sequence) in sequences.clone().iter().enumerate() {
                if row_str.contains(sequence) {
                    res[i] = true;
                }
            }
        }

        // Check columns for a winning sequence
        let n = self.board[0].len();
        for col_idx in 0..n {
            let col_str: String = self.board.iter().map(|row| row[col_idx]).collect();
            for (i, sequence) in sequences.clone().iter().enumerate() {
                if col_str.contains(sequence) {
                    res[i] = true;
                }
            }
        }

        // Check all possible diagonals for a winning sequence
        let diagonals = get_diagonal_strings(&self.board);

        for diagonal in diagonals {
            for (i, sequence) in sequences.clone().iter().enumerate() {
                if diagonal.contains(sequence) {
                    res[i] = true;
                }
            }
        }
        // No winning sequence found
        match self.game_type {
            GameType::TootAndOtto if self.user_otto_toot.as_str() == "TOOT" => {
                return (res[1], res[0])
            }
            _ => return (res[0], res[1]),
        }

        // }
        // (res[0], res[1])
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
            winners: (false, false),
            user_otto_toot: "None".to_string(),
            t_selected: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.board = vec![
                    vec!['_'; self.num_cols.try_into().unwrap()];
                    self.num_rows.try_into().unwrap()
                ];
                self.winners = (false, false);

                true
            }
            Msg::UserMove { col, choice } => {
                self.perform_move(col, choice);
                self.winners = self.is_win();

                self.user_turn = !self.user_turn;

                true
            }
            Msg::ComputerMove { col, choice } => {
                self.perform_move(col, choice);
                self.winners = self.is_win();

                self.user_turn = !self.user_turn;
                true
            }
            Msg::RenderAgain => true,
            Msg::ChangeSel => {
                wasm_logger::init(wasm_logger::Config::default());
                self.t_selected = !self.t_selected;
                info!("T selected: {}", self.t_selected);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut board = vec![];

        let disabled = self.winners.0
            || self.winners.1
            || !self.user_turn
            || ((self.game_type == GameType::TootAndOtto) && (self.user_otto_toot == "None"));

        let choice = match self.game_type {
            GameType::Connect4 => 'R',
            GameType::TootAndOtto => {
                if self.t_selected {
                    'T'
                } else {
                    'O'
                }
            }
        };
        for i in 0..self.num_cols {
            let onclick = link.callback(move |_| {
                return Msg::UserMove {
                    col: i as usize,
                    choice,
                };
            });

            let col: Vec<char> = self.board.iter().map(|row| row[i as usize]).collect();
            board.push(html! {
                <button {disabled} class="column" onclick={onclick}>

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
        let subtitle = if self.winners.0 || self.winners.1 {
            if self.winners.1 {
                format!("{} wins!", self.player2)
            } else {
                format!("{} wins!", self.player1)
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

        let mut board_classses = vec!["grid"];

        if disabled {
            board_classses.push("disabled-board");
        }

        let toototto = match self.game_type {
            GameType::Connect4 => html! {},
            GameType::TootAndOtto => match self.user_otto_toot.as_str() {
                "None" => html! {
                    <div class="content">
                        <div class="content__container">
                        <p class="content__container__text">{"You are: "}</p>
                        <ul class="content__container__list">
                            <li class="content__container__list__item">{"TOOT"}</li>
                            <li class="content__container__list__item">{"OTTO"}</li>
                        </ul>
                        </div>
                    </div>
                },

                _ => html! {
                    <div>
                        <div class="content">
                            <div class="content__container">
                            <p class="content__container__text">{format!{"You are: {}", self.user_otto_toot.as_str()}}</p>
                            </div>
                        </div>
                        <div style="display: flex; flex-direction: row; justify-content: center;">
                            <button onclick={ctx.link().callback(|_| Msg::ChangeSel)} disabled={self.t_selected} class="circle button" style="background-color: #FFFFFF;">{"T"}</button>
                            <button onclick={ctx.link().callback(|_| Msg::ChangeSel)} disabled={!self.t_selected} class="circle button" style="background-color: #FFFFFF;">{"O"}</button>
                        </div>
                    </div>

                },
            },
        };

        html! {
            <div class="game-container">
            <h1 class="title">{ title }</h1>
            { toototto}

            <h2 class="subtitle">{ subtitle }</h2>
            <div class={classes!(board_classses)}>{ board }</div>
            <button class="restart" onclick={link.callback(|_| Msg::Reset)}>{"Restart"}</button>
        </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && self.game_type == GameType::TootAndOtto {
            let options = vec!["TOOT", "OTTO"];
            self.user_otto_toot = options[if Math::random() < 0.5 { 0 } else { 1 }].to_string();
            let link = ctx.link().clone();
            Timeout::new(3000, move || {
                link.send_message(Msg::RenderAgain);
            })
            .forget();
        }

        if !self.user_turn && !self.winners.0 && !self.winners.1 {
            let (_, col, choice) = self.alpha_beta_minmax(false, 5, i32::MIN, i32::MAX);

            let msg = Msg::ComputerMove { col, choice };

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
        <Game player1={"Ali".to_string()} player2={"AI - Medium".to_string()} game_type={GameType::Connect4} />
    }
}
#[function_component]
pub fn TootOtto() -> Html {
    html! {
        <Game player1={"Rupin".to_string()} player2={"AI - Medium".to_string()} game_type={GameType::TootAndOtto} />
    }
}
