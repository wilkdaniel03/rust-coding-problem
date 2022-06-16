use std::collections::HashMap;

struct TwoNumberSum;

#[derive(Debug)]
enum SumResult {
    Found([i8; 2]),
    NotFound([i8; 0]),
}

#[allow(dead_code)]
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

    fn find_using_hash_map(vector: Vec<i8>, target_sum: i8) -> SumResult {
        let mut result: SumResult = SumResult::NotFound([]);
        let mut nums: HashMap<i8, bool> = HashMap::new();
        for num in vector.iter() {
            let possible_match = target_sum - num;
            if nums.contains_key(&possible_match) {
                result = SumResult::Found([*num, possible_match]);
            } else {
                nums.insert(*num, true);
            }
        }
        result
    }

    fn find_using_pointers(vector: &mut Vec<i8>, target_sum: i8) -> SumResult {
        vector.sort();
        let mut result: SumResult = SumResult::NotFound([]);
        let mut smallest_idx = 0;
        let mut largest_idx = vector.len() - 1;
        while smallest_idx < largest_idx && largest_idx < vector.len() {
            let possible_result = vector[smallest_idx] + vector[largest_idx];
            if possible_result < target_sum {
                smallest_idx += 1;
            } else if possible_result > target_sum {
                largest_idx -= 1;
            } else {
                result = SumResult::Found([
                    vector[smallest_idx],
                    vector[largest_idx]
                ]);
                break;
            }
        }
        result
    }
}

fn main() {
    let mut v: Vec<i8> = vec!(3,5,-4,8,11,1,-1,6);
    // println!("{:?}", TwoNumberSum::find_using_nested_loop(v, 10));
    // println!("{:?}", TwoNumberSum::find_using_hash_map(v, 10));
    println!("{:?}", TwoNumberSum::find_using_pointers(&mut v, 10));
}
