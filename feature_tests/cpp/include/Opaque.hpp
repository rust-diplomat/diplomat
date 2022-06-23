#ifndef Opaque_HPP
#define Opaque_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "Opaque.h"
}

class Opaque;
struct MyStruct;

/**
 * A destruction policy for using Opaque with std::unique_ptr.
 */
struct OpaqueDeleter {
  void operator()(capi::Opaque* l) const noexcept {
    capi::Opaque_destroy(l);
  }
};
class Opaque {
 public:
  static Opaque new_();
  void assert_struct(MyStruct s) const;
  inline const capi::Opaque* AsFFI() const { return this->inner.get(); }
  inline capi::Opaque* AsFFIMut() { return this->inner.get(); }
  inline Opaque(capi::Opaque* i) : inner(i) {}
  Opaque() = default;
  Opaque(Opaque&&) noexcept = default;
  Opaque& operator=(Opaque&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Opaque, OpaqueDeleter> inner;
};

#include "MyStruct.hpp"

inline Opaque Opaque::new_() {
  return Opaque(capi::Opaque_new());
}
inline void Opaque::assert_struct(MyStruct s) const {
  MyStruct diplomat_wrapped_struct_s = s;
  capi::Opaque_assert_struct(this->inner.get(), capi::MyStruct{ .a = diplomat_wrapped_struct_s.a, .b = diplomat_wrapped_struct_s.b, .c = diplomat_wrapped_struct_s.c, .d = diplomat_wrapped_struct_s.d, .e = diplomat_wrapped_struct_s.e, .f = diplomat_wrapped_struct_s.f });
}
#endif
