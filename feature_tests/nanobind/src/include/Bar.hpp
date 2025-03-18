#ifndef Bar_HPP
#define Bar_HPP

#include "Bar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    const diplomat::capi::Foo* Bar_foo(const diplomat::capi::Bar* self);
    
    
    void Bar_destroy(Bar* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const Foo& Bar::foo() const {
  auto result = diplomat::capi::Bar_foo(this->AsFFI());
  return *Foo::FromFFI(result);
}

inline const diplomat::capi::Bar* Bar::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Bar*>(this);
}

inline diplomat::capi::Bar* Bar::AsFFI() {
  return reinterpret_cast<diplomat::capi::Bar*>(this);
}

inline const Bar* Bar::FromFFI(const diplomat::capi::Bar* ptr) {
  return reinterpret_cast<const Bar*>(ptr);
}

inline Bar* Bar::FromFFI(diplomat::capi::Bar* ptr) {
  return reinterpret_cast<Bar*>(ptr);
}

inline void Bar::operator delete(void* ptr) {
  diplomat::capi::Bar_destroy(reinterpret_cast<diplomat::capi::Bar*>(ptr));
}


#endif // Bar_HPP
