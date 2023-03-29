use client::Leaderboard;
use gloo_net::http::Request;
use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn LeaderBoard() -> Html {

    let users = use_state(|| vec![]);
    {
        let users = users.clone();
        use_effect_with_deps(move |_| {
            let users = users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users: Vec<Leaderboard> = Request::get("http://127.0.0.1:8000/leaderboard")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                users.set(fetched_users);
            });
            || ()
        }, ());
    }

    let user_col: Vec<Leaderboard> = users.iter().map(|user| user.clone()).collect();
    let size = users.len();

    html! {
        <>
            <h1>{"Leaderboard"}</h1>
            <p>{format!("Top {} users", "todo")}</p>
            <div>
                <table>
                    <thead>
                        <tr>
                            <th>{"Rank"}</th>
                            <th>{"Username"}</th>
                            <th>{"Wins"}</th>
                            <th>{"Losses"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for user_col.iter().enumerate().map(|(i, user)| {
                                html! {
                                    <tr>
                                        <td>{i+1}</td>
                                        <td>{&user.username}</td>
                                        <td>{&user.wins}</td>
                                        <td>{&user.losses}</td>
                                    </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
            </div>
        </>
    }
}
