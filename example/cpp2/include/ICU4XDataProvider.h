#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XDataProvider.d.h"

namespace capi {


extern "C" {

ICU4XDataProvider* ICU4XDataProvider_new_static();

struct ICU4XDataProvider_returns_result_result { bool is_ok;};
struct ICU4XDataProvider_returns_result_result ICU4XDataProvider_returns_result();


void ICU4XDataProvider_destroy(ICU4XDataProvider* self);

} // extern "C"

} // namespace capi

#endif // ICU4XDataProvider_H
