#ifndef Float64Vec_HPP
#define Float64Vec_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<Float64Vec> inline new_(const std::span<double> v) {
	// TODO
}
	void inline fill_slice(std::span<double> v) const {
	// TODO
}
	void inline set_value(const std::span<double> new_slice) {
	// TODO
}
inline capi::Float64Vec* AsFFI() {
	return reinterpret_cast::<capi::Float64Vec>(this);
}
inline ~Float64Vec() {
	capi::Float64Vec_destroy(AsFFI());
}


#endif // Float64Vec_HPP
