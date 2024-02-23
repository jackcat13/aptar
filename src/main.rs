mod components;
mod helpers;
mod model;

use linked_hash_map::LinkedHashMap;
use web_sys::Storage;
use crate::components::login::{LoginComponent, TOKEN_KEY};
use crate::helpers::local_storage::local_storage;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::model::discord_token::DiscordToken;
use crate::model::discord_token_params::DiscordTokenParams;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    html! {
        <>
            {header()}

            {footer()}
        </>
    }
}

fn header() -> Html {
    html! {
        <>
            <div class="absolute top-0 right-0 m-5">
                {"Welcome "}
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <>
            <div class="sticky bottom-0 w-full">
                <div class="px-8 place-items-center flex justify-center">

                </div>
            </div>
        </>
    }
}

fn not_found() -> Html {
    html! { <h1>{ "404" }</h1> }
}

fn login() -> Html {
    html! {
        <>
            <LoginComponent />
            {footer()}
        </>
    }
}

fn switch(routes: Route) -> Html {
    let local_storage = local_storage();
    let user_token = local_storage.get(TOKEN_KEY).unwrap();
    match user_token {
        Some(_) => match routes {
            Route::Home => home(),
            Route::NotFound => not_found(),
        },
        None => login(),
    }
}

#[function_component(App)]
fn app() -> Html {
    let local_storage = local_storage();
    let location = use_location().expect("Failed to load location");
    let queries: LinkedHashMap<String, String> =
        location.query::<LinkedHashMap<String, String>>().expect("Could not retrieve get parameters");
    let discord_code = queries.get("code");
    if let Some(discord_code) = discord_code {
        let discord_code = discord_code.clone();
        wasm_bindgen_futures::spawn_local(async move {
            discord_token(&discord_code, &local_storage).await;
        });
    }
    html! {
        <>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </>
    }
}

async fn discord_token(discord_code: &String, storage: &Storage) {
    let storage = storage.clone();
    let client = reqwest::Client::new();
    let params = DiscordTokenParams {
        client_id: "1210617983534112868".to_string(),
        client_secret: "4W2tNGDnogNjNrIZSRzXd0jqKgKvQTVe".to_string(),
        grant_type: "authorization_code".to_string(),
        code: discord_code.to_string(),
        redirect_url: "https://jackcat13.github.io/aptar/".to_string(),
    };
    let res = client.post("https://discordapp.com/api/oauth2/token").json(&params).send().await;
    let res = res.expect("Failed to get Discord access token");
    let token: DiscordToken = res.json().await.expect("Failed to parse Discord token");
    storage.set(TOKEN_KEY, token.access_token.as_str()).expect("Failed to store discord token");
}

fn main() {
    yew::Renderer::<App>::new().render();
}
