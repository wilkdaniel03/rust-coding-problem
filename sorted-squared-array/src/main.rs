struct SortedSquaredArray;

#[allow(dead_code)]
impl SortedSquaredArray {
    fn sort_using_algorithm(vector: Vec<i8>) -> Vec<i8> {
        let mut squared_vector: Vec<i8> = Vec::with_capacity(vector.len());
        for num in vector.iter() {
            squared_vector.push(*num * *num);
        }
        squared_vector.sort();
        squared_vector
    }
}

fn main() {
    let v: Vec<i8> = vec!(1,2,3,5,6,8,9);
    println!("{:?}", SortedSquaredArray::sort_using_algorithm(v));
}
