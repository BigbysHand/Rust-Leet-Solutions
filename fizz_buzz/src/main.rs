
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut fizz_buzz: Vec<String> = Vec::new();
    for i in 1..n+1{
        let mut join: String = "".to_owned();
        
        if i % 3 != 0 && i % 5 != 0{
            fizz_buzz.push(i.to_string());
            continue;
        }
        if i % 3 == 0{
            join.push_str("Fizz")
        }
        if i % 5 == 0{
            join.push_str("Buzz")
        }

        fizz_buzz.push(String::from(join));

    }
    return fizz_buzz;
}

fn main(){
    
}
