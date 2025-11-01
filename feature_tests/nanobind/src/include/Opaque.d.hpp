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
} // namespace capi
} // namespace

namespace somelib {
class Opaque {
public:

  inline static std::unique_ptr<somelib::Opaque> new_();

  inline static std::unique_ptr<somelib::Opaque> try_from_utf8(std::string_view input);

  inline static somelib::diplomat::result<std::unique_ptr<somelib::Opaque>, somelib::diplomat::Utf8Error> from_str(std::string_view input);

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

    inline const somelib::capi::Opaque* AsFFI() const;
    inline somelib::capi::Opaque* AsFFI();
    inline static const somelib::Opaque* FromFFI(const somelib::capi::Opaque* ptr);
    inline static somelib::Opaque* FromFFI(somelib::capi::Opaque* ptr);
    inline static void operator delete(void* ptr);
private:
    Opaque() = delete;
    Opaque(const somelib::Opaque&) = delete;
    Opaque(somelib::Opaque&&) noexcept = delete;
    Opaque operator=(const somelib::Opaque&) = delete;
    Opaque operator=(somelib::Opaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Opaque_D_HPP
