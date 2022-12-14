fn main()
{
    println!("{:?}\r\n", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}\r\n", two_sum(vec![3, 2, 4], 6));
    println!("{:?}\r\n", two_sum(vec![3, 3], 6));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let i_end = nums.len() - 1;
    let j_end = nums.len();

    for i in 0..i_end
    {
        for j in i+1..j_end
        {
            println!("i:{}, j:{} / {} + {} = {}", i, j, nums[i], nums[j], target);

            if nums[i] + nums[j] == target {
                result.push(i as i32);
                result.push(j as i32);

                return  result;
            }
        }
    }

    return result;
}
