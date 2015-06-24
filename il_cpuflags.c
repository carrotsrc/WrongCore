#include <asm/processor.h>
void il_cpuid(unsigned int id, unsigned int *a, unsigned int *b, unsigned int *c, unsigned int *d) {
	cpuid(id, a, b, c, d);
}
