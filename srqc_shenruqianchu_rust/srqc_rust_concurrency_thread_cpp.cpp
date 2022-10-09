#include<iostream>
#include<stdlib.h>
#include<thread>
#include<string>

#define COUNT 1000000
volatile int g_num = 0;

void thread1() {
    for (int i = 0; i < COUNT; i++) {
        g_num++;
    }
}

void thread2() {
for (int i = 0; i < COUNT; i++) {
    g_num--;
    }
}

int main( int argc, char *argv[] ) {
    std::thread t1(thread1);
    std::thread t2(thread2);
    t1.join();
    t2.join();
    std::cout << "finial value: " << g_num << std::endl;
    return 0;

}