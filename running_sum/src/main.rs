pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut result = Vec::new();
    for i in nums {
        sum += i;
        result.push(sum);
    }
    result
}

fn main() {
    let nums = vec![0, 2, 3, 4, 5];
    println!("{:?}", running_sum(nums));
}
