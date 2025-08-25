#ifndef ns_RenamedDeprecatedOpaque_HPP
#define ns_RenamedDeprecatedOpaque_HPP

#include "RenamedDeprecatedOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {

    void namespace_DeprecatedOpaque_destroy(RenamedDeprecatedOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const ns::capi::RenamedDeprecatedOpaque* ns::RenamedDeprecatedOpaque::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedDeprecatedOpaque*>(this);
}

inline ns::capi::RenamedDeprecatedOpaque* ns::RenamedDeprecatedOpaque::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedDeprecatedOpaque*>(this);
}

inline const ns::RenamedDeprecatedOpaque* ns::RenamedDeprecatedOpaque::FromFFI(const ns::capi::RenamedDeprecatedOpaque* ptr) {
  return reinterpret_cast<const ns::RenamedDeprecatedOpaque*>(ptr);
}

inline ns::RenamedDeprecatedOpaque* ns::RenamedDeprecatedOpaque::FromFFI(ns::capi::RenamedDeprecatedOpaque* ptr) {
  return reinterpret_cast<ns::RenamedDeprecatedOpaque*>(ptr);
}

inline void ns::RenamedDeprecatedOpaque::operator delete(void* ptr) {
  ns::capi::namespace_DeprecatedOpaque_destroy(reinterpret_cast<ns::capi::RenamedDeprecatedOpaque*>(ptr));
}


#endif // ns_RenamedDeprecatedOpaque_HPP
