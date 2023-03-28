use yew::prelude::*;

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn RegisterForm() -> Html {
    html! {
        <form>
            <div>
                <label for="username">{"Username"}</label>
                <input type="text" name="username" />
            </div>
            <div>
                <label for="email">{"Email Address"}</label>
                <input type="text" name="email" />
            </div>
            <div>
                <label for="password">{"Password"}</label>
                <input type="password" name="password" />
            </div>
            <div>
                <button type="submit">{"Register"}</button>
            </div>
        </form>
    }
}
