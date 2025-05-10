#![allow(dead_code)]
#![deny(soft_unstable)]
#![feature(asm)]
#![feature(deadline_api)]
#![feature(if_let_guard)]

use std::{collections::LinkedList, future, io::Error, mem::size_of_val, path::Path, thread};

use sixty_challenge_days::impls::{
    file::File,
    product::{OrderStatus, Product, ProductCategory},
};

fn main() -> Result<(), Error> {
    // let day_1_path = Path::new("day-1.md");
    // let file = File::open_file(day_1_path)?;
    // file.print_file();
    let sub_thr = thread::spawn(|| {
        let id = thread::current().id();
        println!("Hello from the sub thread, my ID is: {id:#?}");
    });

    let id = thread::current().id();
    println!("Hello from the main thread, my ID is: {id:#?}");
    let y = {
        let mut x = [1, 2, 3, 4, 5];
        let mid = 2;
        let len = x.len();
        for _ in 0..mid {
            let temp = x[0];
            x.copy_within(1..len, 0);

            x[len - 1] = temp;
        }

        x
    };
    println!("{y:?}");

    let width1 = 30;
    let height1 = 50;
    let dimensions = (width1, height1);
    let mut order = OrderStatus::Pending;

    println!("{:?}", order);
    order.update_order_status("payment_received");
    println!("{:?}", order);

    let mut product = Product::new(
        "Computer".to_string(),
        1250.00,
        ProductCategory::Electronics,
    );

    product.display_product();

    println!("size of Product {}", size_of_val(&product));
    product.price = 542313.00;
    product.name = "Lucas".to_string();
    println!("size of Product {}", size_of_val(&product));

    let mut list = LinkedList::new();

    list.push_back(Expense(54.242));
    list.push_back(Expense(32.242));
    list.push_front(Expense(879.242));
    list.push_front(Expense(1233.52));

    println!("{:?}", list);
    list.pop_back();
    list.pop_front();
    println!("{:?}", list);

    println!("Head: {:?}", list.front());
    println!("Tail: {:?}", list.back());
    Expense::print_total(list.into_iter());

    Ok(())
}

#[derive(Debug)]
struct Expense(f32);

impl Expense {
    fn print_total<T: Iterator<Item = Self>>(iter: T) {
        let sum = iter.fold(0f32, |sum, curr| sum + curr.0);

        println!("Total values: {}", sum);
    }
}
