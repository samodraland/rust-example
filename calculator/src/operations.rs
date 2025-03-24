pub fn summation(nums: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for num in nums {
        sum += num
    }
    sum as f32 //no semicolon means it will return the value
}

pub fn subtraction(nums: &Vec<i32>) -> f32 {
    let mut sub: i32 = nums[0];
    //start looping from index 1
    for num in nums.iter().skip(1) {
        sub -= num;
    }
    sub as f32
}

pub fn division(nums: &Vec<i32>) -> f32 {
    let mut div = nums[0] as f32;
    if nums[nums.len() - 1] == 0 {
        panic!("Cannot divide by 0")
    } else {
        for &num in nums.iter().skip(1) {
            div /= num as f32;
            div = (div * 100.0).round() / 100.0;
        }
        div as f32
    }
}

pub fn multiplication(nums: &Vec<i32>) -> f32 {
    let mut multiply: i32 = 1;
    for num in nums {
        multiply *= num
    }
    multiply as f32
}
