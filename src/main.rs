use std::ops::Add;

fn main() {
    let test_vector:Vec<(f64,f64)> = vec![(3.0, -2.0), (1.0, 1.0), (-2.0,-2.0)];
    let vec_sum = add_vectors_2d(test_vector);
    println!("{:?}", vec_sum);

    let test_vector:Vec<(i32,i32,i32)> = vec![(3, -3, 2), (1, 1,2), (3,3,-2)];
    let vec_sum = add_vectors_3d(test_vector);
    println!("{:?}", vec_sum);

    let translation_vector = (1.0,1.0);
    let mut test_vector = vec![(0.0,0.0),(0.0,1.0),(-3.0,-3.0)];
    let translated_vector = translate_vector_2d(translation_vector,test_vector);
    println!("{:?}", translated_vector);

    let translation_vector = (1,1);
    let mut test_vector = vec![(0,0),(0,1),(-3,-3)];
    let translated_vector = translate_vector_2d(translation_vector,test_vector);
    println!("{:?}", translated_vector);

    let translation_vector = (1.0,1.0,1.0);
    let mut test_vector = vec![(0.0,0.0,1.0),(0.0,1.0,2.4),(-3.0,-3.0,5.1)];
    let translated_vector = translate_vector_3d(translation_vector,test_vector);
    println!("{:?}", translated_vector);

    let translation_vector = (1,1,1);
    let mut test_vector = vec![(0,1,2),(0,1,3),(-3,-3,4)];
    let translated_vector = translate_vector_3d(translation_vector,test_vector);
    println!("{:?}", translated_vector);

}


fn add_vectors_2d<T>(vector: Vec<(T,T)>) -> Option<(T,T)>
    where
        T: Add + Add<Output = T>

{
    vector.into_iter().reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1))
}

fn add_vectors_3d<T>(vector: Vec<(T,T,T)>) -> Option<(T,T,T)>
    where
        T: Add + Add<Output = T>

{
    vector.into_iter().reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1, acc.2 + elem.2))
}

fn translate_vector_2d<T>(translation_vector:(T,T), vector:Vec<(T,T)>) -> Vec<(T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
   vector.into_iter().map(|(x,y)| (x+translation_vector.0,y+translation_vector.1)).collect()

}

fn translate_vector_3d<T>(translation_vector:(T,T,T), vector:Vec<(T,T,T)>) -> Vec<(T, T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y,z)| (x+translation_vector.0,y+translation_vector.1, z + translation_vector.2)).collect()
}



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

fn translate_point_2d<T>(translation_vector:(T,T), vector:Vec<(T,T)>) -> Vec<(T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y)| (x+translation_vector.0,y+translation_vector.1)).collect()

}

fn translate_point_3d<T>(translation_vector:(T,T,T), vector:Vec<(T,T,T)>) -> Vec<(T, T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y,z)| (x+translation_vector.0,y+translation_vector.1, z + translation_vector.2)).collect()
}


#[derive(Debug)]
pub struct Point2d<T> {
    x:T,
    y:T,
}
#[derive(Debug)]
pub struct Point3d<T> {
    x:T,
    y:T,
    z:T,
}