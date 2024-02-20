#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub category: ProductCategory,
}

impl Product {
    pub fn new(name: String, price: f32, category: ProductCategory) -> Self {
        Product {
            name,
            price,
            category,
        }
    }
    pub fn display_product(&self) {
        match self.category {
            ProductCategory::Hygiene => {
                println!("Higiene: {}\nPrice: {}", self.name, self.price);
            }
            ProductCategory::Clothing => {
                println!("Clothings: {}\nPrice: {}", self.name, self.price,);
            }
            ProductCategory::HomeGoods => {
                println!("Home Goods: {}\nPrice: {}", self.name, self.price,);
            }
            ProductCategory::Electronics => {
                println!("Electronics: {}\nPrice: {}", self.name, self.price,);
            }
        }
    }
}

#[derive(Debug)]
pub enum ProductCategory {
    Hygiene,
    Clothing,
    HomeGoods,
    Electronics,
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn update_order_status(&mut self, event: &str) -> &mut Self {
        match event {
            "payment_received" => {
                if *self < OrderStatus::Processing {
                    *self = OrderStatus::Processing;
                }
            }
            "shipped" => {
                if *self < OrderStatus::Shipped {
                    *self = OrderStatus::Shipped;
                }
            }
            "cancel_requested" => {
                if *self != OrderStatus::Shipped || *self != OrderStatus::Delivered {
                    *self = OrderStatus::Cancelled;
                }
            }
            _ => (),
        };

        self
    }
}

#[cfg(test)]
mod test {
    use super::{Product, ProductCategory};

    #[test]
    fn when_add_value_in_struct_its_size_increase() {
        let mut product1 = Product::new("".to_string(), 0.00, super::ProductCategory::Clothing);
        let product1_size = std::mem::size_of_val(&product1);

        product1.name = "Lucas Fellippe Alves Pereira".to_string();
        product1.price = 15432.2146;
        product1.category = ProductCategory::Hygiene;
        let product2_size = std::mem::size_of_val(&product1);

        assert_eq!(
            product1_size, product2_size,
            "Yes, the size increase when add values to a struct"
        );
    }
}
