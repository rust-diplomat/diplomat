#ifndef One_HPP
#define One_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "One.h"
}

class One;
class Two;

/**
 * A destruction policy for using One with std::unique_ptr.
 */
struct OneDeleter {
  void operator()(capi::One* l) const noexcept {
    capi::One_destroy(l);
  }
};
class One {
 public:
  static One transitivity(const One& hold, const One& nohold);
  static One cycle(const Two& hold, const One& nohold);
  static One many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold);
  static One return_outlives_param(const Two& hold, const One& nohold);
  static One diamond_top(const One& top, const One& left, const One& right, const One& bottom);
  static One diamond_left(const One& top, const One& left, const One& right, const One& bottom);
  static One diamond_right(const One& top, const One& left, const One& right, const One& bottom);
  static One diamond_bottom(const One& top, const One& left, const One& right, const One& bottom);
  static One diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold);
  static One implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold);
  static One implicit_bounds_deep(const One& explicit, const One& implicit_1, const One& implicit_2, const One& nohold);
  inline const capi::One* AsFFI() const { return this->inner.get(); }
  inline capi::One* AsFFIMut() { return this->inner.get(); }
  inline One(capi::One* i) : inner(i) {}
  One() = default;
  One(One&&) noexcept = default;
  One& operator=(One&& other) noexcept = default;
 private:
  std::unique_ptr<capi::One, OneDeleter> inner;
};

#include "Two.hpp"

inline One One::transitivity(const One& hold, const One& nohold) {
  return One(capi::One_transitivity(hold.AsFFI(), nohold.AsFFI()));
}
inline One One::cycle(const Two& hold, const One& nohold) {
  return One(capi::One_cycle(hold.AsFFI(), nohold.AsFFI()));
}
inline One One::many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold) {
  return One(capi::One_many_dependents(a.AsFFI(), b.AsFFI(), c.AsFFI(), d.AsFFI(), nohold.AsFFI()));
}
inline One One::return_outlives_param(const Two& hold, const One& nohold) {
  return One(capi::One_return_outlives_param(hold.AsFFI(), nohold.AsFFI()));
}
inline One One::diamond_top(const One& top, const One& left, const One& right, const One& bottom) {
  return One(capi::One_diamond_top(top.AsFFI(), left.AsFFI(), right.AsFFI(), bottom.AsFFI()));
}
inline One One::diamond_left(const One& top, const One& left, const One& right, const One& bottom) {
  return One(capi::One_diamond_left(top.AsFFI(), left.AsFFI(), right.AsFFI(), bottom.AsFFI()));
}
inline One One::diamond_right(const One& top, const One& left, const One& right, const One& bottom) {
  return One(capi::One_diamond_right(top.AsFFI(), left.AsFFI(), right.AsFFI(), bottom.AsFFI()));
}
inline One One::diamond_bottom(const One& top, const One& left, const One& right, const One& bottom) {
  return One(capi::One_diamond_bottom(top.AsFFI(), left.AsFFI(), right.AsFFI(), bottom.AsFFI()));
}
inline One One::diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold) {
  return One(capi::One_diamond_and_nested_types(a.AsFFI(), b.AsFFI(), c.AsFFI(), d.AsFFI(), nohold.AsFFI()));
}
inline One One::implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold) {
  return One(capi::One_implicit_bounds(explicit_hold.AsFFI(), implicit_hold.AsFFI(), nohold.AsFFI()));
}
inline One One::implicit_bounds_deep(const One& explicit, const One& implicit_1, const One& implicit_2, const One& nohold) {
  return One(capi::One_implicit_bounds_deep(explicit.AsFFI(), implicit_1.AsFFI(), implicit_2.AsFFI(), nohold.AsFFI()));
}
#endif
