use std::fmt;

#[derive(Debug)]
pub struct Point2d<T>{
    pub x:T,
    pub y:T,
}
#[derive(Debug)]
pub struct Point3d<T>{
    pub x:T,
    pub y:T,
    pub z:T,
}

impl fmt::Display for Point2d<i32>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "x {}, y {}", self.x, self.y)
    }
}

impl fmt::Display for Point2d<f64>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "x {}, y {}", self.x, self.y)
    }
}

impl fmt::Display for Point3d<i32>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "x {}, y {}, z {}", self.x, self.y, self.z)
    }
}