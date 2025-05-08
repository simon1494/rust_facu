pub struct Rectangle {
    length: f64,
    width: f64,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle { length, width }
    }

    fn calculate_perimeter(&self) -> f64 {
        return 2.0 * (self.length + self.width);
    }

    fn calculate_area(&self) -> f64 {
        return self.length * self.width;
    }

    fn is_square(&self) -> bool {
        return self.length == self.width;
    }
}

#[test]
fn test_rectangle_constructor() {
    let width: f64 = 10.0;
    let length: f64 = 25.2;
    let rectangle: Rectangle = Rectangle::new(length, width);

    assert_eq!(width, rectangle.width);
    assert_eq!(length, rectangle.length);
}

#[test]
fn test_calculate_perimeter() {
    let width: f64 = 10.0;
    let length: f64 = 2.0;
    let rectangle: Rectangle = Rectangle::new(length, width);

    assert_eq!(24.0, rectangle.calculate_perimeter());
}

#[test]
fn test_calculate_area() {
    let width: f64 = 10.0;
    let length: f64 = 2.0;
    let rectangle: Rectangle = Rectangle::new(length, width);

    assert_eq!(20.0, rectangle.calculate_area());
}
