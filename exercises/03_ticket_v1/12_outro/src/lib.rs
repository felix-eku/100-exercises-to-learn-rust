pub struct Order {
    product_name: String,
    quantity: u64,
    unit_price: u64, // in cents
}

impl Order {
    pub fn new(product_name: String, quantity: u64, unit_price: u64) -> Self {
        Self::check_name(&product_name);
        Self::check_quantity(quantity);
        Self::check_price(unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u64 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::check_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u64) {
        Self::check_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u64) {
        Self::check_price(unit_price);
        self.unit_price = unit_price;
    }

    pub fn total(&self) -> u64 {
        self.unit_price() * self.quantity()
    }

    fn check_name(product_name: &String) {
        if product_name.is_empty() {
            panic!("Product name cannot be empty")
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes")
        }
    }

    fn check_quantity(quantity: u64) {
        if quantity == 0 {
            panic!("Quantity must be strictly greater than zero")
        }
    }

    fn check_price(unit_price: u64) {
        if unit_price == 0 {
            panic!("Unit price must be strictly greater than zero")
        }
    }
}
