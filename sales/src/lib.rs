#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().map(|name, _| name == &ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        if self.items.is_empty() {
            return Vec::new();
        }
        let mut prices: Vec<f32> = self.items.iter().map(|_, price| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let free_items_count = prices.len() / 3;
        let total_discount: f32 = prices.iter().take(free_items_count).sum();
        let total_price: f32 = prices.iter().sum();
        let discount_percentage = total_discount / total_price;
        let mut receipt: Vec<f32> = prices
            .iter()
            .map(|price| {

                let discounted = price * (1.0 - discount_percentage);
                (discounted * 100.0).round() / 100.0
            })
            .collect();

        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = receipt.clone();

        receipt
    }
}
