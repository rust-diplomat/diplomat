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

#include "Opaque.h"

class Opaque;
struct MyStruct;
struct ImportedStruct;

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

  /**
   * See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
   * 
   * See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
   * 
   * Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
   */
  void assert_struct(MyStruct s) const;
  static size_t returns_usize();
  static ImportedStruct returns_imported();
  inline const capi::Opaque* AsFFI() const { return this->inner.get(); }
  inline capi::Opaque* AsFFIMut() { return this->inner.get(); }
  inline explicit Opaque(capi::Opaque* i) : inner(i) {}
  Opaque() = default;
  Opaque(Opaque&&) noexcept = default;
  Opaque& operator=(Opaque&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Opaque, OpaqueDeleter> inner;
};

#include "MyStruct.hpp"
#include "ImportedStruct.hpp"

inline Opaque Opaque::new_() {
  return Opaque(capi::Opaque_new());
}
inline void Opaque::assert_struct(MyStruct s) const {
  MyStruct diplomat_wrapped_struct_s = s;
  capi::Opaque_assert_struct(this->inner.get(), capi::MyStruct{ .a = diplomat_wrapped_struct_s.a, .b = diplomat_wrapped_struct_s.b, .c = diplomat_wrapped_struct_s.c, .d = diplomat_wrapped_struct_s.d, .e = diplomat_wrapped_struct_s.e, .f = diplomat_wrapped_struct_s.f, .g = static_cast<capi::MyEnum>(diplomat_wrapped_struct_s.g) });
}
inline size_t Opaque::returns_usize() {
  return capi::Opaque_returns_usize();
}
inline ImportedStruct Opaque::returns_imported() {
  capi::ImportedStruct diplomat_raw_struct_out_value = capi::Opaque_returns_imported();
  return ImportedStruct{ .foo = std::move(static_cast<UnimportedEnum>(diplomat_raw_struct_out_value.foo)), .count = std::move(diplomat_raw_struct_out_value.count) };
}
#endif
