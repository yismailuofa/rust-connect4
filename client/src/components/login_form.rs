use client::User;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub set_user: Callback<Option<User>>,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let username_value = use_state(|| "".to_string());
    let password_value = use_state(|| "".to_string());

    let props = props.clone();

    let onclick = move |_| {
        props.set_user.emit(Some(User {
            username: "test".to_string(),
            password: "test".to_string(),
        }));
    };

    html! {
        <form>
            <div>
                <label for="username">{"Username"}</label>
                <input type="text" name="username" />
            </div>
            <div>
                <label for="password">{"Password"}</label>
                <input type="password" name="password" />
            </div>
            <div>
                <button type="submit" {onclick} >{"Login"}</button>
            </div>
        </form>
    }
}
