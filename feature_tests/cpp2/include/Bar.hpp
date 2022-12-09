#ifndef Bar_HPP
#define Bar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "Bar.d.hpp"
#include "Bar.h"




inline const capi::Bar* Bar::AsFFI() const {
	return reinterpret_cast<const capi::Bar*>(this);
}
inline capi::Bar* Bar::AsFFI() {
	return reinterpret_cast<capi::Bar*>(this);
}
inline Bar::~Bar() {
	capi::Bar_destroy(AsFFI());
}


#endif // Bar_HPP
