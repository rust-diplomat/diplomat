#ifndef Two_HPP
#define Two_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




inline capi::Two* AsFFI() {
	return reinterpret_cast::<capi::Two>(this);
}
inline ~Two() {
	capi::Two_destroy(AsFFI());
}


#endif // Two_HPP
