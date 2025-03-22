#ifndef Bar_D_HPP
#define Bar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Foo; }
class Foo;


namespace diplomat {
namespace capi {
    struct Bar;
} // namespace capi
} // namespace

class Bar {
public:

  inline const Foo& foo() const;

  inline const diplomat::capi::Bar* AsFFI() const;
  inline diplomat::capi::Bar* AsFFI();
  inline static const Bar* FromFFI(const diplomat::capi::Bar* ptr);
  inline static Bar* FromFFI(diplomat::capi::Bar* ptr);
  inline static void operator delete(void* ptr);
private:
  Bar() = delete;
  Bar(const Bar&) = delete;
  Bar(Bar&&) noexcept = delete;
  Bar operator=(const Bar&) = delete;
  Bar operator=(Bar&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Bar_D_HPP
