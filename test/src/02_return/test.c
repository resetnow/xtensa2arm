#include <test.h>
#include <stdint.h>

uint32_t f() {
	return 0xaabbccdd;
}

int main() {
	test_finish(f() == 0xaabbccdd);
	return 0;
}
