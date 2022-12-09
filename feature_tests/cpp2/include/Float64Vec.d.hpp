#ifndef Float64Vec_D_HPP
#define Float64Vec_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Float64Vec.d.h"




class Float64Vec {
public:
	static std::unique_ptr<Float64Vec> inline new_(const std::span<double> v);

	void inline fill_slice(std::span<double> v) const;

	void inline set_value(const std::span<double> new_slice);

	inline capi::Float64Vec* AsFFI();

	inline ~Float64Vec();

private:
	Float64Vec() = delete;
};





#endif // Float64Vec_D_HPP
