use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use axum::Json;
use rand::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi()]
pub struct QuotesApiDoc;

pub fn router(_state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::<AppState>::with_openapi(QuotesApiDoc::openapi()).routes(routes!(post_quote))
}

/// Query an ingredient's price-quotes
#[utoipa::path(
        post,
        path = "/query",
        request_body(content = inline(Ingredient), content_type = "application/json"),
        responses(
            (status = OK, description = "Quote recieved", body = Vec<PriceQuote>)
        ),
    )]
#[axum::debug_handler]
async fn post_quote(Json(ingredient): Json<Ingredient>) -> Result<Json<Vec<PriceQuote>>> {
    Ok(Json::from(get_fake_quotes(&ingredient)))
}

/* Mocking bullshit for quotes n shit... */

const RETAILERS: [&str; 4] = ["Aldi", "Lidl", "Sainsburys", "Tesco"];

/* From an ingredient, get a number of fake quotes. */
fn get_fake_quotes(ingredient: &Ingredient) -> Vec<PriceQuote> {
    let seed = calculate_ingredient_hash(ingredient);
    let mut rng = SmallRng::seed_from_u64(seed);
    let num_quotes = rng.random_range(1..=3);
    let mut quotes = vec![];

    for _ in 0..num_quotes {
        quotes.push(get_fake_quote(&mut rng));
    }

    quotes
}

/* Gets one fake quote, using a random source. */
fn get_fake_quote(rng: &mut SmallRng) -> PriceQuote {
    let retailer = RETAILERS
        .choose(rng)
        .expect("This should work!")
        .to_string();

    let mut price_pence: usize = rng.random_range(6..=60) * 5;
    if price_pence.rem_euclid(10) == 0 {
        price_pence -= 1;
    }

    PriceQuote {
        retailer,
        price_pence,
        product_url: "https://example.com".to_string(),
    }
}

/// Gets the hash for an ingredient. Only takes into consideration the type, amount, and unit.
fn calculate_ingredient_hash(ingredient: &Ingredient) -> u64 {
    let mut s = DefaultHasher::new();

    ingredient.name.hash(&mut s);
    ingredient.quantity.to_bits().hash(&mut s);
    ingredient.unit.hash(&mut s);

    s.finish()
}
