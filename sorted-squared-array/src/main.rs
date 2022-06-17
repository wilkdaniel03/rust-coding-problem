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

    fn sort_in_place(vector: Vec<i8>) -> Vec<i8> {
        let mut squared_array: Vec<i8> = vec![0; vector.len()]; 
        let mut start: usize = 0;
        let mut end: usize = vector.len() - 1;
        let mut idx: usize = vector.len() - 1;
        while start <= end {
            if vector[start].abs() > vector[end].abs() {
                squared_array[idx] = vector[start] * vector[start];
                start += 1;
            } else {
                squared_array[idx] = vector[end] * vector[end];
                end -= 1;
            }
            idx -= 1;
        }
        squared_array
    }
}

fn main() {
    let v: Vec<i8> = vec!(1,2,3,5,6,8,9);
    // println!("{:?}", SortedSquaredArray::sort_using_algorithm(v));
    println!("{:?}", SortedSquaredArray::sort_in_place(v));
}
