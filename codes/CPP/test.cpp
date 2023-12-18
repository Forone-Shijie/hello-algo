
#include <iostream>
#include <iterator>
#include <list>
#include <ostream>
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

// int fib(int n){
//     if (n == 1 || n==2){
//         return n-1;
//     }

//     int res = fib(n-1) + fib(n-2);

//     return res;
// }

int arr[] = {1,2,3,4,5};
int length = sizeof(arr)/sizeof(arr[0]);

void print_array(int *array, int length){
    std::cout << "The array is:";

    for (int i=0; i<length; i++){
        std::cout << array[i] << ' ';
    }
    std::cout << "The print ends." << std::endl;
}

//insert one element into array
int insert(int *array, int length,int insert_element, int insert_location){
    for (int i = length; i>insert_location; i--){
        array[i] = array[i-1];
    }

    array[insert_location] = insert_element;

    return *array;
}

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x): val(x), next(nullptr) {}
};

int main(){

    // insert(arr, length, 6, 1);
    // print_array(arr, length);
    ListNode* n0 = new ListNode(1);
    ListNode* n1 = new ListNode(2);
    ListNode* n2 = new ListNode(3);
    ListNode* n3 = new ListNode(5);
    ListNode* n4 = new ListNode(4);

    n0->next = n1;
    n1->next = n2;
    n2->next = n3;
    n3->next = n4;

}