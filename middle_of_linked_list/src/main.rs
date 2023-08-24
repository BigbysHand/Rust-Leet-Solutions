impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pointer = head.clone();
        let mut length: i32 = 0;
        while head != None{
            length += 1;
            head = head.unwrap().next;
        }
        for i in 0..(length /2){
            pointer = pointer.unwrap().next;
        }
        return pointer;
    }
}

fn main() {
    println!("Hello, world!");
}
