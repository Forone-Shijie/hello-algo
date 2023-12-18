// for loop
// fn for_loop(n: i32) -> i32{
//     let mut res = 0;

//     for i in 1..=n{
//         res += i;
//     }

//     res
// }

// fn main(){

//     let result = for_loop(3);
//     println!("result = {0}, {1}", result, for_loop(4));
// }


// fn while_loop(n: i32) -> i32{
//     let mut res = 0;
//     let mut i = 0;

//     while i<=n {
//         res += i;
//         i+=1;
//     }

//     res
// }

// fn recur(n: i32) -> i32{
//     if n == 1{ //the condition of ending
//         return 1; 
//     }

//     let result = n* recur(n-1); // recur method

//     result
// }

// fn fib(n: i32) -> i32{
//     if n==1 || n==2{
//         return n-1;
//     }

//     let res = fib(n-1) + fib(n-2);

//     res
// }

// fn main(){
//     println!("Result = {}", fib(5));
// }
fn insert (array: &mut Vec<i32>, num: i32, index: usize){
    array.insert(index, num)
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)] 
// 使该链表实例能够被printf! {:?}输出
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>, // 指向下一个节点的指针
}

fn main(){
    // let mut arr: Vec<i32> = vec![0; 5];
    // insert(&mut arr, 5, 0);
    // println!("Result : {:?}", arr);

    let tail = Rc::new(RefCell::new(ListNode {val: 3, next: None, }));

    let second = Rc::new(RefCell::new(ListNode {val: 2, next: Some(Rc::clone(&tail)),}));
    let first = Rc::new(RefCell::new(ListNode {val: 1, next: Some(Rc::clone(&tail)), }));

    println!("The first ListNode is {:?}", first);


}


