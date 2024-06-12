#ifndef Bar_D_HPP
#define Bar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class Foo;


namespace capi {
    typedef struct Bar Bar;
}

class Bar {
public:

  inline const Foo& foo() const;

  inline const capi::Bar* AsFFI() const;
  inline capi::Bar* AsFFI();
  inline static const Bar* FromFFI(const capi::Bar* ptr);
  inline static Bar* FromFFI(capi::Bar* ptr);
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
