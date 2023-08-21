pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max: i32 = 0;
    for i in &accounts{

        let mut wealth: i32 = 0;
        for val in i{

            wealth += val;
        }
        if wealth > max{
            max = wealth;
        }
    }
    return max;
}