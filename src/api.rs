use crate::types::Product;
use gloo_net::http::Request;

pub async fn get_products() -> Result<Vec<Product>, gloo_net::Error> {
    let response = Request::get("/products/products.json").send().await?;
    let products: Vec<Product> = response.json().await?;
    Ok(products)
}

pub async fn get_product(id: i32) -> Result<Product, gloo_net::Error> {
    let url = format!("/products/{}.json", id);
    let response = Request::get(&url).send().await?;
    let product: Product = response.json().await?;
    Ok(product)
}