use client::User;
use gloo_net::http::Request;
use yew::prelude::*;

use gloo_dialogs::alert;
use yew_router::prelude::{use_navigator, Link};

use crate::{LoginRoute, MainRoute};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub set_username: Callback<Option<String>>,
}

#[function_component]
pub fn RegisterForm(props: &Props) -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let username_value = use_state(|| "".to_string());
    let password_value = use_state(|| "".to_string());

    let props = props.clone();

    let navigator = use_navigator().unwrap();

    let onclick = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        move |_| {
            let navigator = navigator.clone();
            let set_username = props.set_username.clone();

            let username = username_ref.cast::<web_sys::HtmlInputElement>();

            if let Some(username) = username {
                if username.value().is_empty() {
                    alert("Username is required");
                    return;
                }
                username_value.set(username.value());
            } else {
                alert("Username is required");
                return;
            }

            let password = password_ref.cast::<web_sys::HtmlInputElement>();

            if let Some(password) = password {
                if password.value().is_empty() {
                    alert("Password is required");
                    return;
                }

                password_value.set(password.value());
            } else {
                alert("Password is required");
                return;
            }

            let username = username_ref.cast::<web_sys::HtmlInputElement>();
            let username_value = username.unwrap().value();

            let password = password_ref.cast::<web_sys::HtmlInputElement>();
            let password_value = password.unwrap().value();

            wasm_bindgen_futures::spawn_local(async move {
                let user = User {
                    username: username_value.to_string(),
                    password: password_value.to_string(),
                };

                let result = Request::post("http://127.0.0.1:8000/users/register")
                    .json(&user)
                    .unwrap()
                    .send()
                    .await;

                let response = match result {
                    Ok(response) => response,
                    Err(err) => {
                        alert(&format!("Error: {}", err));
                        return;
                    }
                };
                let status = response.status();
                if status == 500 {
                    alert("Database not available");
                    return;
                }
                if status != 200 {
                    alert("Invalid username or password");
                    return;
                }

                set_username.emit(Some(username_value.to_string()));

                // Redirect to the home page
                navigator.push(&MainRoute::Connect4);
            });
        }
    };

    html! {
        <>
            <div>
                <label for="username">{"Username"}</label>
                <input type="text" name="username" required={true} ref={username_ref}/>
            </div>
            <div>
                <label for="password">{"Password"}</label>
                <input type="password" name="password" required={true} ref={password_ref}/>
            </div>
            <div>
                <button type="submit" {onclick}>{"Register"}</button>
            </div>
            <Link<LoginRoute> to={LoginRoute::Login}>{ "Click here to login" }</Link<LoginRoute>>

        </>
    }
}
