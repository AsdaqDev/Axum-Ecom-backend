use std::sync::Arc;

use crate::modules::auth::service::AuthService;
use crate::modules::products::service::ProductService;
use crate::modules::orders::service::OrderService;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub auth_service: AuthService,
    pub product_service: ProductService,
    pub order_service: OrderService,
}
