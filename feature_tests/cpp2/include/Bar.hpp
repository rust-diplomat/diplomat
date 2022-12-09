#ifndef Bar_HPP
#define Bar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Bar.d.hpp"
#include "Bar.h"




inline capi::Bar* Bar::AsFFI() {
	return reinterpret_cast<capi::Bar*>(this);
}
inline Bar::~Bar() {
	capi::Bar_destroy(AsFFI());
}


#endif // Bar_HPP
