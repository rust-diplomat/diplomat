#ifndef One_D_HPP
#define One_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "One.d.h"


class Two;


class One {
public:
	static std::unique_ptr<One> inline transitivity(const One& hold, const One& nohold);

	static std::unique_ptr<One> inline cycle(const Two& hold, const One& nohold);

	static std::unique_ptr<One> inline many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold);

	static std::unique_ptr<One> inline return_outlives_param(const Two& hold, const One& nohold);

	static std::unique_ptr<One> inline diamond_top(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> inline diamond_left(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> inline diamond_right(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> inline diamond_bottom(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> inline diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold);

	static std::unique_ptr<One> inline implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold);

	static std::unique_ptr<One> inline implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold);

	inline capi::One* AsFFI();

	inline ~One();

private:
	One() = delete;
};





#endif // One_D_HPP
