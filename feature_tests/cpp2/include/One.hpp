#ifndef One_HPP
#define One_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "One.d.hpp"
#include "One.h"




inline std::unique_ptr<One> One::transitivity(const One& hold, const One& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::cycle(const Two& hold, const One& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::return_outlives_param(const Two& hold, const One& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::diamond_top(const One& top, const One& left, const One& right, const One& bottom) {
	// TODO
}
inline std::unique_ptr<One> One::diamond_left(const One& top, const One& left, const One& right, const One& bottom) {
	// TODO
}
inline std::unique_ptr<One> One::diamond_right(const One& top, const One& left, const One& right, const One& bottom) {
	// TODO
}
inline std::unique_ptr<One> One::diamond_bottom(const One& top, const One& left, const One& right, const One& bottom) {
	// TODO
}
inline std::unique_ptr<One> One::diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold) {
	// TODO
}
inline std::unique_ptr<One> One::implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold) {
	// TODO
}
inline capi::One* One::AsFFI() {
	return reinterpret_cast<capi::One*>(this);
}
inline One::~One() {
	capi::One_destroy(AsFFI());
}


#endif // One_HPP
