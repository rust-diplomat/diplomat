#ifndef One_HPP
#define One_HPP

#include "One.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Two.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::One* One_transitivity(const diplomat::capi::One* hold, const diplomat::capi::One* nohold);
    
    diplomat::capi::One* One_cycle(const diplomat::capi::Two* hold, const diplomat::capi::One* nohold);
    
    diplomat::capi::One* One_many_dependents(const diplomat::capi::One* a, const diplomat::capi::One* b, const diplomat::capi::Two* c, const diplomat::capi::Two* d, const diplomat::capi::Two* nohold);
    
    diplomat::capi::One* One_return_outlives_param(const diplomat::capi::Two* hold, const diplomat::capi::One* nohold);
    
    diplomat::capi::One* One_diamond_top(const diplomat::capi::One* top, const diplomat::capi::One* left, const diplomat::capi::One* right, const diplomat::capi::One* bottom);
    
    diplomat::capi::One* One_diamond_left(const diplomat::capi::One* top, const diplomat::capi::One* left, const diplomat::capi::One* right, const diplomat::capi::One* bottom);
    
    diplomat::capi::One* One_diamond_right(const diplomat::capi::One* top, const diplomat::capi::One* left, const diplomat::capi::One* right, const diplomat::capi::One* bottom);
    
    diplomat::capi::One* One_diamond_bottom(const diplomat::capi::One* top, const diplomat::capi::One* left, const diplomat::capi::One* right, const diplomat::capi::One* bottom);
    
    diplomat::capi::One* One_diamond_and_nested_types(const diplomat::capi::One* a, const diplomat::capi::One* b, const diplomat::capi::One* c, const diplomat::capi::One* d, const diplomat::capi::One* nohold);
    
    diplomat::capi::One* One_implicit_bounds(const diplomat::capi::One* explicit_hold, const diplomat::capi::One* implicit_hold, const diplomat::capi::One* nohold);
    
    diplomat::capi::One* One_implicit_bounds_deep(const diplomat::capi::One* explicit_, const diplomat::capi::One* implicit_1, const diplomat::capi::One* implicit_2, const diplomat::capi::One* nohold);
    
    
    void One_destroy(One* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<One> One::transitivity(const One& hold, const One& nohold) {
  auto result = diplomat::capi::One_transitivity(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::cycle(const Two& hold, const One& nohold) {
  auto result = diplomat::capi::One_cycle(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold) {
  auto result = diplomat::capi::One_many_dependents(a.AsFFI(),
    b.AsFFI(),
    c.AsFFI(),
    d.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::return_outlives_param(const Two& hold, const One& nohold) {
  auto result = diplomat::capi::One_return_outlives_param(hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_top(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = diplomat::capi::One_diamond_top(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_left(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = diplomat::capi::One_diamond_left(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_right(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = diplomat::capi::One_diamond_right(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_bottom(const One& top, const One& left, const One& right, const One& bottom) {
  auto result = diplomat::capi::One_diamond_bottom(top.AsFFI(),
    left.AsFFI(),
    right.AsFFI(),
    bottom.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold) {
  auto result = diplomat::capi::One_diamond_and_nested_types(a.AsFFI(),
    b.AsFFI(),
    c.AsFFI(),
    d.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold) {
  auto result = diplomat::capi::One_implicit_bounds(explicit_hold.AsFFI(),
    implicit_hold.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline std::unique_ptr<One> One::implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold) {
  auto result = diplomat::capi::One_implicit_bounds_deep(explicit_.AsFFI(),
    implicit_1.AsFFI(),
    implicit_2.AsFFI(),
    nohold.AsFFI());
  return std::unique_ptr<One>(One::FromFFI(result));
}

inline const diplomat::capi::One* One::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::One*>(this);
}

inline diplomat::capi::One* One::AsFFI() {
  return reinterpret_cast<diplomat::capi::One*>(this);
}

inline const One* One::FromFFI(const diplomat::capi::One* ptr) {
  return reinterpret_cast<const One*>(ptr);
}

inline One* One::FromFFI(diplomat::capi::One* ptr) {
  return reinterpret_cast<One*>(ptr);
}

inline void One::operator delete(void* ptr) {
  diplomat::capi::One_destroy(reinterpret_cast<diplomat::capi::One*>(ptr));
}


#endif // One_HPP
