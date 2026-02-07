use crate::api;
use crate::components::ProductCard;
use crate::types::{CartProduct, Product};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub struct Home {
    products: Vec<Product>,
    error: Option<String>,
    loading: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetProducts);
        Self {
            products: vec![],
            error: None,
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProducts => {
                self.loading = true;
                let link = ctx.link().clone();
                spawn_local(async move {
                    match api::get_products().await {
                        Ok(products) => link.send_message(Msg::GetProductsSuccess(products)),
                        Err(e) => link.send_message(Msg::GetProductsError(e.to_string())),
                    }
                });
                true
            }
            Msg::GetProductsSuccess(products) => {
                self.products = products;
                self.loading = false;
                true
            }
            Msg::GetProductsError(error) => {
                self.error = Some(error);
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let products: Html = self
            .products
            .iter()
            .map(|product| {
                html! {
                    <ProductCard 
                        product={product.clone()} 
                        on_add_to_cart={ctx.props().on_add_to_cart.clone()}
                    />
                }
            })
            .collect();

        if self.loading {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else if self.error.is_some() {
            html! {
                <div>
                    <span>{"Error loading products! :("}</span>
                </div>
            }
        } else {
            html! {
                <div class="product_card_list">{products}</div>
            }
        }
    }
}