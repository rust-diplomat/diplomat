#ifndef One_HPP
#define One_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "One.h"
#include "Two.d.hpp"

#include "One.d.hpp"


inline std::unique_ptr<One> One::transitivity(const One& hold, const One& nohold) {
  auto result = capi::One_transitivity(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::cycle(const Two& hold, const One& nohold) {
  auto result = capi::One_cycle(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold) {
  auto result = capi::One_many_dependents(a.AsFFI(),
    b.AsFFI(),
    c.AsFFI(),
    d.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::return_outlives_param(const Two& hold, const One& nohold) {
  auto result = capi::One_return_outlives_param(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_top(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = capi::One_diamond_top(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_left(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = capi::One_diamond_left(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_right(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = capi::One_diamond_right(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_bottom(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = capi::One_diamond_bottom(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold) {
  auto result = capi::One_diamond_and_nested_types(a.AsFFI(),
    b.AsFFI(),
    c.AsFFI(),
    d.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold) {
  auto result = capi::One_implicit_bounds(explicit_hold.AsFFI(),
    implicit_hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline std::unique_ptr<One> One::implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold) {
  auto result = capi::One_implicit_bounds_deep(explicit_.AsFFI(),
    implicit_1.AsFFI(),
    implicit_2.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr(One::FromFFI(result));
}

inline const capi::One* One::AsFFI() const {
  return reinterpret_cast<const capi::One*>(this);
}

inline capi::One* One::AsFFI() {
  return reinterpret_cast<capi::One*>(this);
}

inline const One* One::FromFFI(const capi::One* ptr) {
  return reinterpret_cast<const One*>(ptr);
}

inline One* One::FromFFI(capi::One* ptr) {
  return reinterpret_cast<One*>(ptr);
}

inline One::~One() {
  capi::One_destroy(AsFFI());
}


#endif // One_HPP
