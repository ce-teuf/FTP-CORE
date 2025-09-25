#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t add(int32_t a, int32_t b);

int32_t safe_divide(int32_t numerator, int32_t denominator);

}  // extern "C"
