#ifndef Float64Vec_D_HPP
#define Float64Vec_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "Float64Vec.d.h"




class Float64Vec {
public:
	inline static std::unique_ptr<Float64Vec> new_(const std::span<double> v);

	inline void fill_slice(std::span<double> v) const;

	inline void set_value(const std::span<double> new_slice);

	inline capi::Float64Vec* AsFFI();

	inline ~Float64Vec();

private:
	Float64Vec() = delete;
};





#endif // Float64Vec_D_HPP
