use std::collections::HashMap;

use crate::coffee_operation::operation_manager::CoffeeOperationManager;

use super::product_model::CoffeeProduct;

pub struct CoffeeShopInventory {
    inventory: HashMap<String, CoffeeProduct>,
}

impl CoffeeShopInventory {
    pub fn new() -> Self {
        Self {
            inventory: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: CoffeeProduct) {
        self.inventory.insert(product.name.to_string(), product);
    }

    pub fn view_products(&self) -> Vec<&CoffeeProduct> {
        self.inventory.values().collect()
    }

    pub fn remove_product(&mut self, product_name: String) -> bool {
        self.inventory.remove(&product_name).is_some()
    }

    pub fn update_product(&mut self, name: String, price: f64) -> bool {
        match self.inventory.get_mut(&name) {
            Some(item) => {
                item.price = price;
                true
            }
            None => false,
        }
    }
}

impl<T> CoffeeOperationManager<T> for CoffeeShopInventory {
    fn new() -> Self {
        todo!()
    }

    fn add_item(&mut self, item: T) {
        todo!()
    }

    fn view_item(&self) {
        todo!()
    }
}
