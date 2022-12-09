#ifndef RefListParameter_HPP
#define RefListParameter_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




inline capi::RefListParameter* AsFFI() {
	return reinterpret_cast::<capi::RefListParameter>(this);
}
inline ~RefListParameter() {
	capi::RefListParameter_destroy(AsFFI());
}


#endif // RefListParameter_HPP
