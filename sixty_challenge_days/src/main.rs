use std::path::Path;

use impls::file::File;

mod impls;

fn main() {
    let day_1_path = Path::new("day-1.md");
    let file = File::open_file(day_1_path);
    file.print_file();
}
