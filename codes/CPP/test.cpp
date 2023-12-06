
#include <iostream>
#include <sstream>
#include <string>

// std::string nestedForloop(int n){
//     std::ostringstream res;

//     for (int i=1; i<=n; ++i){
//         for (int j=1; j<=n; ++j){
//             res << "(" << i << ", " << j << "), ";
//         }
//     }
//     return res.str();
// }

int fib(int n){
    if (n == 1 || n==2){
        return n-1;
    }

    int res = fib(n-1) + fib(n-2);

    return res;
}

int main(){
    int n = 5;
    std::cout << "Result = " << fib(n) << std::endl;
}