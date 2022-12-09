#ifndef Two_HPP
#define Two_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Two.d.hpp"
#include "Two.h"




inline capi::Two* Two::AsFFI() {
	return reinterpret_cast<capi::Two*>(this);
}
inline Two::~Two() {
	capi::Two_destroy(AsFFI());
}


#endif // Two_HPP
