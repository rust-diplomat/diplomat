#ifndef Bar_HPP
#define Bar_HPP

#include "Bar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Foo.hpp"


namespace capi {
    extern "C" {
    
    const ::capi::Foo* Bar_foo(const ::capi::Bar* self);
    
    
    void Bar_destroy(Bar* self);
    
    } // extern "C"
}
inline const Foo& Bar::foo() const {
  auto result = capi::Bar_foo(this->AsFFI());
  return *Foo::FromFFI(result);
}

inline const ::capi::Bar* Bar::AsFFI() const {
  return reinterpret_cast<const ::capi::Bar*>(this);
}

inline ::capi::Bar* Bar::AsFFI() {
  return reinterpret_cast<::capi::Bar*>(this);
}

inline const Bar* Bar::FromFFI(const ::capi::Bar* ptr) {
  return reinterpret_cast<const Bar*>(ptr);
}

inline Bar* Bar::FromFFI(::capi::Bar* ptr) {
  return reinterpret_cast<Bar*>(ptr);
}

inline void Bar::operator delete(void* ptr) {
  capi::Bar_destroy(reinterpret_cast<::capi::Bar*>(ptr));
}


#endif // Bar_HPP
