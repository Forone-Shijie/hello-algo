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

fn fib(n: i32) -> i32{
    if n==1 || n==2{
        return n-1;
    }

    let res = fib(n-1) + fib(n-2);

    res
}

fn main(){
    println!("Result = {}", fib(5));
}
