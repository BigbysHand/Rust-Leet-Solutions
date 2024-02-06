
pub fn is_palindrome(x: i32) -> bool
{
    if x < 10 && x >= 0
    {
        return true;
    }
    if x < 0
    {
        return false;
    }
    //int -> stack array
    let digits: Vec<_> = x
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let mut front = 0;
    let mut back = digits.len() - 1;

    for i in 0..digits.len() / 2
    {
        if (digits[front] != digits[back])
        {
            return false;
        }
        front += 1;
        back -= 1;
    }

    return true;

}

