pub trait CoffeeOperationManager<T> {
    fn add_item(&mut self, item: T);

    fn view_item(&self) -> Vec<&T>;

    fn remove_item(&mut self, name: String) -> bool;

    fn update_item(&mut self, item: T) -> bool;
}
