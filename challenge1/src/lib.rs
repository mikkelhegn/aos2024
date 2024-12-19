use std::hash::{DefaultHasher, Hash, Hasher};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;
use urlencoding::decode;

mod bindings;

#[derive(Deserialize, Serialize)]
struct Wishlist {
    name: String,
    items: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Non {
    name: String,
    score: u8,
}

#[derive(Deserialize, Serialize)]
struct GiftSuggestion {
    name: String,
    age: u8,
    likes: String,
}

#[derive(Deserialize, Serialize)]
struct GiftSuggestions {
    name: String,
    #[serde(rename(serialize = "giftSuggestions"))]
    gift_suggestions: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge1(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/api/wishlists", get_wishlists);
    router.post("/api/wishlists", post_wishlists);
    router.get("/api/naughty-or-nice/:name", non);
    router.post("/api/generate-gift-suggestions", generate_gift_suggestions);
    router.handle(req)
}

fn non(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let name = String::from(decode(params.get("name").expect("No param")).expect("decode failed"));
    let result = Non {
        name,
        score: bindings::deps::advent_of_spin::challenge_two::non::get_naughty_or_nice_score(0),
    };

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(serde_json::to_vec(&result).expect("Failed to deserialize result object"))
        .build())
}

fn post_wishlists(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let wishlish: Wishlist = match serde_json::from_slice(req.body()) {
        Ok(w) => w,
        Err(_) => {
            println!("Failed to deserialize the body to a Wishlist");
            return Ok(Response::builder().status(400).build());
        }
    };

    let store = Store::open_default().expect("Failed to open the default store");

    let mut hasher = DefaultHasher::new();
    let _ = &wishlish.name.hash(&mut hasher);
    let key = hasher.finish().to_string();
    store
        .set_json(key, &wishlish)
        .expect("Failed to store wishlist");

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .status(201)
        .build())
}

fn get_wishlists(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let store = Store::open_default().expect("Failed to open the default store");
    let keys = store.get_keys().expect("Failed to get keys from the store");

    let wishlists: Vec<Wishlist> = keys
        .iter()
        .filter_map(|k| -> Option<Wishlist> { store.get_json(k).unwrap() })
        .collect();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serde_json::to_vec(&wishlists)?)
        .build())
}

fn generate_gift_suggestions(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let suggestion: GiftSuggestion = match serde_json::from_slice(req.body()) {
        Ok(gs) => gs,
        Err(_) => {
            println!("Failed to deserialize the body to a Gift Suggestion");
            return Ok(Response::builder().status(400).build());
        }
    };

    let ai_suggestion = bindings::deps::components::advent_of_spin::generator::suggest(
        &suggestion.name,
        suggestion.age,
        &suggestion.likes,
    )
    .expect("Didn't get any good suggestions");

    let gift_suggestions = GiftSuggestions {
        name: ai_suggestion.name,
        gift_suggestions: ai_suggestion.suggestions,
    };

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serde_json::to_vec(&gift_suggestions)?)
        .build())
}
