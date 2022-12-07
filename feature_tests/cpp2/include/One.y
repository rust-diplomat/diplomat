#ifndef One_HPP
#define One_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class Two;


class One {
public:
	static std::unique_ptr<One> transitivity(const One& hold, const One& nohold);

	static std::unique_ptr<One> cycle(const Two& hold, const One& nohold);

	static std::unique_ptr<One> many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold);

	static std::unique_ptr<One> return_outlives_param(const Two& hold, const One& nohold);

	static std::unique_ptr<One> diamond_top(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> diamond_left(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> diamond_right(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> diamond_bottom(const One& top, const One& left, const One& right, const One& bottom);

	static std::unique_ptr<One> diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold);

	static std::unique_ptr<One> implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold);

	static std::unique_ptr<One> implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold);

	inline capi::One AsFFI() {
		return reinterpret_cast::<capi::One>(this);
	}

	~One() {
		One_destroy(AsFFI());
	}

private:
	One() = delete;
}





#endif // One_HPP
