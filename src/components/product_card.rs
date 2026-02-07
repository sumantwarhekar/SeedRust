use crate::components::AtcButton;
use crate::route::Route;
use crate::types::Product;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

#[function_component(ProductCard)]
pub fn product_card(props: &Props) -> Html {
    html! {
        <div class="product_card_container">
            <Link<Route> to={Route::ProductDetail { id: props.product.id }} classes="product_card_anchor">
                <img class="product_card_image" src={props.product.image.clone()}/>
                <div class="product_card_name">{&props.product.name}</div>
                <div class="product_card_price">{"$"}{&props.product.price}</div>
            </Link<Route>>
            <AtcButton product={props.product.clone()} on_add_to_cart={props.on_add_to_cart.clone()} />
        </div>
    }
}