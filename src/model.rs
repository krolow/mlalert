use diesel::prelude::*;

#[derive(Queryable)]
pub struct Product {
    pub id: Uuid,
    pub url: String,
    pub status: ProductStatus,
    pub sales: Int4,
    pub seller: Seller,
}

pub struct Seller {
    pub id: Uuid,
    pub url: String,
    pub lastYearSales: Int4,
    pub reputation: Int4,
}

enum ProductStatus {
    available,
    out,
    pause,
}
