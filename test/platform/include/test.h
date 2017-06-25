#include <stdint.h>
#include <stdbool.h>

static inline void test_finish(bool pass) {
    extern volatile uint32_t __test_pass;
    __test_pass = pass;
}
