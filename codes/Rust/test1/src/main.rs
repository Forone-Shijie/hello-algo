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


fn while_loop(n: i32) -> i32{
    let mut res = 0;
    let mut i = 0;

    while i<=n {
        res += i;
        i+=1;
    }

    res
}

fn main(){
    println!("Result = {}", while_loop(5));
}
