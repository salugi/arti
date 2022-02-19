use structs::points::*;
use operations::vector_operations::{PointOperations, TupleOperations};
fn main() {
   let point2:Point2d<i32> = Point2d{
       x:1,
       y:3,
   };
   let point3:Point3d<i32> = Point3d{
    x:1,
    y:3,
    z:4,
    };

    let point2d_vector:Vec<Point2d<i32>> = vec![
        Point2d{
            x:2,
            y:3,
        },
        Point2d{
            x:2,
            y:3,
        }];
    let point3d_vector:Vec<Point3d<i32>> = vec![
            Point3d{
                x:2,
                y:3,
                z:3,
            },
            Point3d{
                x:2,
                y:3,
                z:3,
            }];

    let added2 = PointOperations::add_2d_points(point2d_vector).unwrap();
    let added3 = PointOperations::add_3d_points(point3d_vector).unwrap();

    println!("{}", added2);
    println!("{}", added3); 


}
