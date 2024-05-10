# Day 2: Rust Structs and Enums - 19-02-2024

## Exercises

### Understanding Struct and Enums

The Rust Programming Language Book:

- Chapter 5: Structs: https://doc.rust-lang.org/book/ch05-00-structs.html
- Chapter 6: Enums: https://doc.rust-lang.org/book/ch06-00-enums.html

Focus: Don't skim! Engage actively with code examples in the book and try them out yourself.

### Practical Implementation

#### Exercise 1: Product Catalog
1. Struct Design: Define a struct named ``Product`` with these fields:
   - name: String
   - price: f32 (or another floating-point type suitable for currency)
   - category: Use an enum you create named ``ProductCategory`` with variants like "Electronics", "Clothing", "HomeGoods" (make more if you like!).

2. Creation: Create several instances (at least 3) of your Product struct, setting meaningful values for each field.
3. Printing: Write a simple function (maybe ``display_product``) that neatly prints the information of a product you pass to it.

#### Exercise 2: State Machine

1. Scenario: Model an order fulfillment process. Create an enum ``OrderStatus`` with these variants: ``Pending, Processing, Shipped, Delivered, Cancelled ``

2. Transition Function: Write a function ``update_order_status`` that takes:
   - A mutable reference to a variable representing the current state(like ``current_status: &mut OrderStatus``);
   - An **event** represented by a simple string (e.g., "payment_received", "shipped", "cancel_requested").
3. Logic: Inside a function, use a ``match`` statement on the event. Update the ``current_status`` to the next logical state only if it's a valid transition (you decide the allowed transitions!). Feel free to print a simple message like "Order Updated!"


#### Things to Ponder
##### Memory Layout:
1. When you add fields to a struct, does its size in memory increase? Experiment to get an intuition.
2. Is the data for an enum variant stored with the enum, or does the enum act like a label?

#### Ask and Explore!
Search for examples of how structs and enums are used in small Rust projects on GitHub.

#### Show Me
Your Code: Post your Rust code with attempts at both exercises. It's totally okay if you don't get everything right the first time!
Your Explanation: In your own words, explain these concepts to me:
- Why a value can only have one owner at a time.
- The key difference between a regular variable and a reference.
- Scenarios where you would likely use a mutable vs. an immutable reference

## Answers

#### Things to Ponder
##### Memory Layout:
No, it sizes hasn't increased! That because Rust compile knows the size of Struct and allocate a portion of memory for it. So even we've not assigned some value to struct's fields, the size of struct remains the same. It's like a bank safe, event though it doesn't contain value, the size is the same.
I wrote a function to validate that.
```rs
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
```
No, the data is stored with enum. Enum is a data type, so it contains values and has a length in the memory. It's not only a label.


#### Ask and Explore!
##### Search for examples of how structs and enums are used in small Rust projects on GitHub.
- Project: [surrealDB](https://github.com/surrealdb/indxdb/blob/main/src/db.rs)
- code:
```rs
pub struct Db {
	pub(crate) lk: Arc<Mutex<()>>,
	pub(crate) ds: Rexie,
}

impl Db {
	// Start a new transaction
	pub async fn begin(&self, write: bool) -> Result<Tx, Error> {
		match write {
			true => match self.ds.transaction(&["kv"], TransactionMode::ReadWrite) {
				Ok(tx) => match tx.store("kv") {
					Ok(st) => Ok(Tx::new(tx, st, write, Some(self.lk.clone().lock_owned().await))),
					Err(_) => Err(Error::TxError),
				},
				Err(_) => Err(Error::TxError),
			},
			false => match self.ds.transaction(&["kv"], TransactionMode::ReadOnly) {
				Ok(tx) => match tx.store("kv") {
					Ok(st) => Ok(Tx::new(tx, st, write, None)),
					Err(_) => Err(Error::TxError),
				},
				Err(_) => Err(Error::TxError),
			},
		}
	}
}
```

In this code, it matches the value of ``write``, then matches if the ``self.ds.transaction`` arms to ``Ok`` or ``Err``. If ok, it creates a new ``Tx``, depends on the ``write`` value.

### Exercise 1: Transfer of Ownership
```rs
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
        println!(
            "Name: {}\nPrice: {}\nCategory: {:?}",
            self.name, self.price, self.category
        );
    }
}

#[derive(Debug)]
pub enum ProductCategory {
    Hygiene,
    Clothing,
    HomeGoods,
    Electronics,
}

```

### Exercise 2: References & Borrowing
```rs
#[derive(PartialEq, Eq, Debug)]
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
                if *self == OrderStatus::Pending {
                    *self = OrderStatus::Processing;
                }
            }
            "shipped" => {
                if *self == OrderStatus::Processing {
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

```

Great, I'm looking forward to Day 3. Thank you. Here are my answers.

#### Exploring GitHub: SurrealDB

##### Why Structs
The code is organized with two fields, one is a ``Arc`` Mutex and other is a struct ``Rexie``. Maybe the "lk" and "ds" was set together to allow "ls" lock the thread when Rexie it's running a transaction. So the struct groups related types that will work together.

Using ``TransactionMode`` allows passing only valid mode values to the transaction. With enums, the specific part of code is more visible and error-prone by using ``match`` pattern in an enum, because the compiler will force to handler all cases in the enum. If it was a String, the compiler not set this restriction

#### Exercise 2: Order Status
I could use the trait ``PartialOrd`` and ``Ord`` in the enum and using a logic something like this ``if *self == OrderStatus::Pending { *self = OrderStatus::Processing;}``. Since the enum not is ordered, I can compare if an order is less than other and set the correct value.

### My questions
The ``Option<T>`` enum is like Haskell ``Maybe``. It's curious how Enum is Rust is so powerfull than others languages. It has more feature and capabilities, and we can even add methods for it. I'm not sure if my idea in exercise 2 is correct, but I'm curious