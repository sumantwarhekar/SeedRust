use crate::types::Product;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

#[function_component(AtcButton)]
pub fn atc_button(props: &Props) -> Html {
    let product = props.product.clone();
    let on_add_to_cart = props.on_add_to_cart.clone();
    
    let onclick = Callback::from(move |_| {
        on_add_to_cart.emit(product.clone());
    });

    html! {
        <button class="product_atc_button" {onclick}>{"Add To Cart"}</button>
    }
}