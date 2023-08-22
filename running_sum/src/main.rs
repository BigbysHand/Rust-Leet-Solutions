pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut tmp: i32 = 0;
    let mut roll: i32;
    let mut out: Vec<i32> = Vec::new();
    for num in nums{
        roll = num + tmp;
        tmp = roll;
        out.push(roll);
    }
    return out;
}

fn main(){
    
}