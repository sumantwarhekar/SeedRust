use crate::api;
use crate::components::AtcButton;
use crate::types::Product;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub struct ProductDetail {
    product: Option<Product>,
    error: Option<String>,
    loading: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    GetProduct,
    GetProductSuccess(Product),
    GetProductError(String),
}

impl Component for ProductDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetProduct);
        Self {
            product: None,
            error: None,
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProduct => {
                self.loading = true;
                let link = ctx.link().clone();
                let id = ctx.props().id;
                spawn_local(async move {
                    match api::get_product(id).await {
                        Ok(product) => link.send_message(Msg::GetProductSuccess(product)),
                        Err(e) => link.send_message(Msg::GetProductError(e.to_string())),
                    }
                });
                true
            }
            Msg::GetProductSuccess(product) => {
                self.product = Some(product);
                self.loading = false;
                true
            }
            Msg::GetProductError(error) => {
                self.error = Some(error);
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref product) = self.product {
            html! {
                <div class="product_detail_container">
                    <img class="product_detail_image" src={product.image.clone()}/>
                    <div class="product_card_name">{&product.name}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&product.description}</div>
                    <div class="product_card_price">{"$"}{&product.price}</div>
                    <AtcButton product={product.clone()} on_add_to_cart={ctx.props().on_add_to_cart.clone()} />
                </div>
            }
        } else if self.loading {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            html! {
                <div>
                    <span>{"Error loading product! :("}</span>
                </div>
            }
        }
    }
}