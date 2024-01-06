#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    fn _increment_mut(&mut self) {
        self.x += 1;
        self.y += 1;
    }

    fn increment_owned(mut self: Box<Point>) -> Box<Point> {
        self.x += 1;
        self.y += 1;
        self
    }
}


#[derive(Debug, Clone, Copy)]
struct TPoint(u32, u32);

/*
impl Clone for Point {
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}
*/

fn main() {
    let p: Box<Point> = Box::new(Point::new(0, 0));
    let q: Box<Point> = p.increment_owned();
    let r: Box<Point>  = q.clone();
    println!("{:?} {:?}", q, r);

    let mut p = TPoint(0, 0);
    p.0 = 1;
    p.1 = 2;
    let q = p;
    println!("{:?} {:?}", p, q);
}
