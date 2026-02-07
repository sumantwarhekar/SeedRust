use crate::components::Navbar;
use crate::types::{CartProduct, Product};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, ProductDetail};
use crate::route::Route;

#[derive(Default)]
pub struct App {
    cart_products: Vec<CartProduct>,
}

pub enum Msg {
    AddToCart(Product),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddToCart(product) => {
                let cart_product = self
                    .cart_products
                    .iter_mut()
                    .find(|cp: &&mut CartProduct| cp.product.id == product.id);

                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                } else {
                    self.cart_products.push(CartProduct {
                        product: product.clone(),
                        quantity: 1,
                    })
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_add_to_cart = ctx.link().callback(Msg::AddToCart);
        let cart_products = self.cart_products.clone();

        html! {
            <BrowserRouter>
                <Navbar cart_products={cart_products.clone()} />
                <Switch<Route> render={move |route| {
                    let cart_products = cart_products.clone();
                    let on_add_to_cart = on_add_to_cart.clone();
                    match route {
                        Route::ProductDetail { id } => html! {
                            <ProductDetail id={id} on_add_to_cart={on_add_to_cart} />
                        },
                        Route::HomePage => html! {
                            <Home cart_products={cart_products} on_add_to_cart={on_add_to_cart} />
                        },
                        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
                    }
                }} />
            </BrowserRouter>
        }
    }
}