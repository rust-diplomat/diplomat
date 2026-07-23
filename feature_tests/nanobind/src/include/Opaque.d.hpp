#ifndef SOMELIB_Opaque_D_HPP
#define SOMELIB_Opaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Opaque; }
class Opaque;
struct ImportedStruct;
struct MyStruct;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Opaque;
    extern "C" {
    void Opaque_destroy(Opaque* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Opaque;
using OpaqueRef = somelib::diplomat::Ref<Opaque, const somelib::capi::Opaque>;
using OpaqueRefMut = somelib::diplomat::Ref<Opaque, somelib::capi::Opaque>;

class Opaque : public somelib::diplomat::OpaquePointer<Opaque, somelib::capi::Opaque, somelib::capi::Opaque_destroy> {
public:

  inline static somelib::Opaque new_();

  inline static somelib::diplomat::Optional<somelib::Opaque> try_from_utf8(std::string_view input);

  inline static somelib::diplomat::result<somelib::Opaque, somelib::diplomat::Utf8Error> from_str(std::string_view input);

  inline std::string get_debug_str() const;
  template<typename W>
  inline void get_debug_str_write(W& writeable_output) const;

  /**
   * See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
   *
   * See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
   *
   * Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
   */
  inline void assert_struct(somelib::MyStruct s) const;

  inline static size_t returns_usize();

  inline static somelib::ImportedStruct returns_imported();

  inline static int8_t cmp();

};

} // namespace
#endif // SOMELIB_Opaque_D_HPP
