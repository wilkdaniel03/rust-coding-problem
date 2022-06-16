struct ValidateSubsequence;

#[allow(dead_code)]
impl ValidateSubsequence {
    fn check_using_while(vector: Vec<i8>, sequence: Vec<i8>) -> bool {
        let mut vector_idx = 0;
        let mut sequence_idx = 0;
        while vector_idx < vector.len() && sequence_idx < sequence.len() {
            if vector[vector_idx] == sequence[sequence_idx] {
                sequence_idx += 1;
            }
            vector_idx += 1;
        }
        sequence_idx == sequence.len()
    }

    fn check_using_for(vector: Vec<i8>, sequence: Vec<i8>) -> bool {
        let mut sequence_idx = 0;
        for vec_num in vector.iter() {
            if sequence_idx == sequence.len() {
                break;
            }
            if *vec_num == sequence[sequence_idx] {
                sequence_idx += 1;
            }
        }
        sequence_idx == sequence.len()
    }
}

fn main() {
    let v: Vec<i8> = vec!(5,1,22,25,6,-1,8,10);
    let s: Vec<i8> = vec!(1,6,-1,10);
    // println!("{:?}", ValidateSubsequence::check_using_while(v, s));
    println!("{:?}", ValidateSubsequence::check_using_for(v, s));
}
