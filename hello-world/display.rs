use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// TestCase List
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{0}: {1}", count, v)?;
        }


        write!(f, "]")
    }
}



fn main() {

    println!("Structure {}", Structure(23));

    println!("Display {}", MinMax(23, 12));
    println!("Debug {:?}", MinMax(23, 12));

    println!("Display {}", Point2D{x: 1.23, y: 2.13});
    println!("Debug {:?}", Point2D{x: 1.23, y: 2.13});

    // Activity
    let real = 3.3;
    let imag = 7.2;

    let complex = Complex{real, imag};

    println!("Display {}", complex);
    println!("Debug {:?}", complex);

    let v = List(vec![1,2,3]);
    println!("{}", v);
}