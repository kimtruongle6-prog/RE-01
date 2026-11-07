#include <iostream>

int sum_to_n(int n) {
    int sum = 0;
    for (int i = 1; i <= n; ++i) {
        sum += i;
    }
    return sum;
}

int main() {
    volatile int n = 10;
    std::cout << sum_to_n(n) << '\n';
    return 0;
}