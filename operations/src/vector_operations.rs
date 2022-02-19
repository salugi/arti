use std::ops::Add;

use structs::points::{Point2d, Point3d};

pub struct TupleOperations;

impl TupleOperations{
/*

Tuples

 */

pub fn add_vectors_2d<T>(vector: Vec<(T,T)>) -> Option<(T,T)>
    where
        T: Add + Add<Output = T>

{
    vector.into_iter().reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1))
}

pub fn add_vectors_3d<T>(vector: Vec<(T,T,T)>) -> Option<(T,T,T)>
    where
        T: Add + Add<Output = T>

{
    vector.into_iter().reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1, acc.2 + elem.2))
}

pub fn translate_vector_2d<T>(translation_vector:(T,T), vector:Vec<(T,T)>) -> Vec<(T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y)| (x+translation_vector.0,y+translation_vector.1)).collect()

}

pub  fn translate_vector_3d<T>(translation_vector:(T,T,T), vector:Vec<(T,T,T)>) -> Vec<(T, T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y,z)| (x+translation_vector.0,y+translation_vector.1, z + translation_vector.2)).collect()
}

}





pub struct PointOperations;

impl PointOperations{
/*

Points

 */

pub fn add_2d_points<T>(point_vector:Vec<Point2d<T>>)->Option<Point2d<T>>
    where T: std::ops::Add<Output = T>
{
    point_vector.into_iter().reduce(|acc, elem| Point2d {x:acc.x + elem.x,y: acc.y + elem.y})
}
pub fn sub_2d_points<T>(point_vector:Vec<Point2d<T>>)->Option<Point2d<T>>
    where T: std::ops::Sub<Output = T>
{
    point_vector.into_iter().reduce(|acc, elem| Point2d {x:acc.x - elem.x,y: acc.y - elem.y})
}

pub fn add_3d_points<T>(point_vector:Vec<Point3d<T>>)->Option<Point3d<T>>
    where T: std::ops::Add<Output = T>
{
    point_vector.into_iter().reduce(|acc, elem| Point3d {x:acc.x + elem.x,y: acc.y + elem.y,z:acc.z+elem.z})
}
pub fn sub_3d_points<T>(point_vector:Vec<Point3d<T>>)->Option<Point3d<T>>
    where T: std::ops::Sub<Output = T>
{
    point_vector.into_iter().reduce(|acc, elem| Point3d {x:acc.x - elem.x,y: acc.y - elem.y,z:acc.z-elem.z})
}


pub fn translate_2d_points<T>(translation_point:Point2d<T>,point_vector:Vec<Point2d<T>>)->Vec<Point2d<T>>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    point_vector.into_iter().map(|point| Point2d{ x:point.x+translation_point.x,y:point.y+translation_point.y}).collect()

}

pub fn translate_3d_points<T>(translation_point:Point3d<T>,point_vector:Vec<Point3d<T>>)->Vec<Point3d<T>>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    point_vector.into_iter().map(|point| Point3d{ x:point.x+translation_point.x,y:point.y+translation_point.y,z:point.z+translation_point.z}).collect()

}
}