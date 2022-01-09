pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str {
        return "Geometry";
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
    fn name(&self) -> &str {
        return "Rectangle";
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub bottom: u32,
    pub height: u32,
}

impl Geometry for Triangle {
    fn area(&self) -> f64 {
        self.bottom as f64 * self.height as f64 * 0.5
    }
    fn name(&self) -> &str {
        return "Triangle";
    }
}
