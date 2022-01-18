use std::ops::Add;

fn main() {
    let test_vector:Vec<(f64,f64)> = vec![(3.0, -2.0), (1.0, 1.0), (-2.0,-2.0)];
    let vec_sum = add_vectors(test_vector);
    println!("{:?}", vec_sum);

    let translation_vector = (1.0,1.0);
    let mut test_vector = vec![(0.0,0.0),(0.0,1.0),(-3.0,-3.0)];
    let translated_vector = translate_vector(translation_vector,test_vector);
    println!("{:?}", translated_vector);

    let translation_vector = (1,1);
    let mut test_vector = vec![(0,0),(0,1),(-3,-3)];
    let translated_vector = translate_vector(translation_vector,test_vector);
    println!("{:?}", translated_vector);

}

fn add_vectors<T>(vector: Vec<(T,T)>) -> Option<(T,T)>
    where
        T: Add + Add<Output = T>

{
    vector.into_iter().reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1))
}

fn translate_vector<T>(translation_vector:(T,T), vector:Vec<(T,T)>) -> Vec<(T, T)>
    where T:std::ops::Add +  std::ops::Add<Output = T > + std::marker::Copy
{
    vector.into_iter().map(|(x,y)| (x+translation_vector.0,y+translation_vector.1)).collect()

}
