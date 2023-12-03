
#include <iostream>
#include <sstream>
#include <string>

std::string nestedForloop(int n){
    std::ostringstream res;

    for (int i=1; i<=n; ++i){
        for (int j=1; j<=n; ++j){
            res << "(" << i << ", " << j << "), ";
        }
    }
    return res.str();
}

int main(){
    int n = 5;
    std::cout << nestedForloop(n) << std::endl;
}