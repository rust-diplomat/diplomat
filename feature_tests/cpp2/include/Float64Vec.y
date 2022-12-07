#ifndef Float64Vec_HPP
#define Float64Vec_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class Float64Vec {
public:
	static std::unique_ptr<Float64Vec> new_(const std::span<double> v);

	void fill_slice(std::span<double> v) const;

	void set_value(const std::span<double> new_slice);

	inline capi::Float64Vec AsFFI() {
		return reinterpret_cast::<capi::Float64Vec>(this);
	}

	~Float64Vec() {
		Float64Vec_destroy(AsFFI());
	}

private:
	Float64Vec() = delete;
}





#endif // Float64Vec_HPP
