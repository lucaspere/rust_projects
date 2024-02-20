pub fn area(width: u32, height: u32) -> u32 {
    width * height
}

pub fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        Shapes::Quadract(4);

        self.height * self.width
    }
}

enum Shapes {
    Quadract(u8),
    Triangule(u8),
    Irregular,
}

fn teste(s: Shapes) {
    match s {
        Shapes::Quadract(v) => todo!(),
        Shapes::Triangule(v) => todo!(),
        Shapes::Irregular => todo!(),
    }
}
