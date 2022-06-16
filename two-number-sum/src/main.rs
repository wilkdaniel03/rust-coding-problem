struct TwoNumberSum;

#[derive(Debug)]
enum SumResult {
    Found([i8; 2]),
    NotFound([i8; 0]),
}

impl TwoNumberSum {
    fn find_using_nested_loop(vector: Vec<i8>, target_sum: i8) -> SumResult {
        let mut result: SumResult = SumResult::NotFound([]);
        for i in 0..vector.len() {
            let firstnum = vector[i];
            for j in (i + 1)..vector.len() {
                let secondnum = vector[j];
                
                if firstnum + secondnum == target_sum {
                    result = SumResult::Found([firstnum, secondnum]);
                }
            }
        }
        result
    }
}

fn main() {
    let v: Vec<i8> = vec!(3,5,-4,8,11,1,-1,6);
    println!("{:?}", TwoNumberSum::find_using_nested_loop(v, 10));
}
