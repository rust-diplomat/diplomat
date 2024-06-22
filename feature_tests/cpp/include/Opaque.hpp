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
  static std::optional<Opaque> try_from_utf8(const std::string_view input);

  /**
   * Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).
   */
  static Opaque from_str(const std::string_view input);
  template<typename W> void get_debug_str_to_write(W& write) const;
  std::string get_debug_str() const;

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
  static int8_t cmp();
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
inline std::optional<Opaque> Opaque::try_from_utf8(const std::string_view input) {
  auto diplomat_optional_raw_out_value = capi::Opaque_try_from_utf8(input.data(), input.size());
  std::optional<Opaque> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = Opaque(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
inline Opaque Opaque::from_str(const std::string_view input) {
  return Opaque(capi::Opaque_from_str(input.data(), input.size()));
}
template<typename W> inline void Opaque::get_debug_str_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::Opaque_get_debug_str(this->inner.get(), &write_writer);
}
inline std::string Opaque::get_debug_str() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::Opaque_get_debug_str(this->inner.get(), &diplomat_write_out);
  return diplomat_write_string;
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
inline int8_t Opaque::cmp() {
  return capi::Opaque_cmp();
}
#endif
