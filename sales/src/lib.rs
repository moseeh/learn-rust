/// A simple shopping system with 'buy three, get one free' promotion implemented via a distributed discount.

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
    /// (product name, unit price)
    pub items: Vec<(String, f32)>,
    /// generated receipt prices after promotion
    pub receipt: Vec<f32>,
}

impl Cart {
    /// Initializes an empty cart.
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    /// Inserts an item from the store into the cart by product name.
    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some((_, price)) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((ele, *price));
        } else {
            panic!("Product '{}' not found in store", ele);
        }
    }

    /// Generates a receipt applying the "buy three, get one free" promotion by
    /// distributing the total free value across all items.
    /// Returns a sorted Vec<f32> of post-discount prices (rounded to two decimals).
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Extract prices
        let prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        let n = prices.len();

        // Calculate how many free items
        let free_count = n / 3;

        // Compute receipt prices
        let mut receipt: Vec<f32>;
        if free_count > 0 {
            // Sum of all prices
            let total: f32 = prices.iter().sum();
            // Sum of the cheapest `free_count` prices
            let mut sorted_prices = prices.clone();
            sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let free_sum: f32 = sorted_prices.iter().take(free_count).sum();

            // Discount rate to apply to all items
            let discount_rate = free_sum / total;

            // Apply distributed discount and round to two decimals
            receipt = prices
                .iter()
                .map(|price| {
                    let discounted = price * (1.0 - discount_rate);
                    (discounted * 100.0).round() / 100.0
                })
                .collect();
        } else {
            // No promotion applies
            receipt = prices
                .iter()
                .map(|p| ((*p * 100.0).round() / 100.0))
                .collect();
        }

        // Sort the receipt, save, and return
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = receipt.clone();
        receipt
    }
}
