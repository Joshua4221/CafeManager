use std::collections::HashMap;

use super::order_model::CustomerOrder;

pub struct CustomerOrderHistory {
    orders: HashMap<String, CustomerOrder>,
}

impl CustomerOrderHistory {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    pub fn place_order(&mut self, order: CustomerOrder) {
        self.orders.insert(order.product_name.clone(), order);
    }

    pub fn view_orders(&self) -> Vec<&CustomerOrder> {
        self.orders.values().collect()
    }

    pub fn remove_orders(&mut self, product_name: String) -> bool {
        self.orders.remove(&product_name).is_some()
    }

    pub fn modify_orders(&mut self, order: CustomerOrder) -> bool {
        match self.orders.get_mut(&order.product_name) {
            Some(details) => {
                details.quantity = order.quantity;
                details.total_price = order.total_price;

                true
            }
            None => false,
        }
    }
}
