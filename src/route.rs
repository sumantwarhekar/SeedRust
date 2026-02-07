use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/product/:id")]
    ProductDetail { id: i32 },
    #[at("/")]
    HomePage,
    #[not_found]
    #[at("/404")]
    NotFound,
}