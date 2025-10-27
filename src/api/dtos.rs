use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTransactionDto {
    pub from: Uuid,
    pub to: Uuid,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub balance: f64,
}
