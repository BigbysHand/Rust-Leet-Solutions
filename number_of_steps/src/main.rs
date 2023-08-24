impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        //assigns nums value to mut var of same dtype + init mut i32 steps to 0
        let mut nums: i32 = num;
        let mut steps: i32 = 0;
        //while nums not 0
        while nums != 0{
            //checks for even/odd
            if nums % 2 == 0{
                nums = nums / 2;
            }
            else{
                nums -= 1;
            }
            //each parse inc steps
            steps += 1;
        }
        return steps;
        
    }
}

fn main() {
}
