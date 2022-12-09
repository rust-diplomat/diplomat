#ifndef RefListParameter_HPP
#define RefListParameter_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "RefListParameter.d.hpp"
#include "RefListParameter.h"




inline capi::RefListParameter* RefListParameter::AsFFI() {
	return reinterpret_cast<capi::RefListParameter*>(this);
}
inline RefListParameter::~RefListParameter() {
	capi::RefListParameter_destroy(AsFFI());
}


#endif // RefListParameter_HPP
