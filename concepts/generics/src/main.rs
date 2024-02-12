struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T,U> Point2<T, U> {
    fn mixup<X1,X2>(self, other: Point2<X1,X2>) -> Point2<T,X2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
}
