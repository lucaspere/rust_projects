use std::{collections::LinkedList, mem::size_of_val, path::Path};

use impls::file::File;

use crate::impls::{
    product::{OrderStatus, Product},
    rectangles::area_tuple,
};

mod impls;

fn main() {
    let day_1_path = Path::new("day-1.md");
    let file = File::open_file(day_1_path);
    file.print_file();

    let width1 = 30;
    let height1 = 50;
    let dimensions = (width1, height1);
    println!(
        "The area of the rectangle is {} square pixels. With Tuples {}",
        impls::rectangles::area(width1, height1),
        area_tuple(dimensions)
    );

    let mut order = OrderStatus::Pending;

    println!("{:?}", order);
    order.update_order_status("payment_received");
    println!("{:?}", order);

    let mut product = Product::new(
        "Computer".to_string(),
        1250.00,
        impls::product::ProductCategory::Electronics,
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
}

#[derive(Debug)]
struct Expense(f32);

impl Expense {
    fn print_total<T: Iterator<Item = Self>>(iter: T) {
        let sum = iter.fold(0f32, |sum, curr| sum + curr.0);

        println!("Total values: {}", sum);
    }
}
