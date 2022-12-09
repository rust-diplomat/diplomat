#ifndef Bar_HPP
#define Bar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




inline capi::Bar* AsFFI() {
	return reinterpret_cast::<capi::Bar>(this);
}
inline ~Bar() {
	capi::Bar_destroy(AsFFI());
}


#endif // Bar_HPP
