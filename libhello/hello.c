#include <hello.h>

void say_hello(const char *name) {
// void say_hello() {
    if (name == NULL) {
        name = "Rust";
    }

    // char *name = "Rust";

    printf("Hello %s, from C!\n", name);
}