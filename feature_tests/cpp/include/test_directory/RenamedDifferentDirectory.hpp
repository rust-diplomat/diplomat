#ifndef test_directory_RenamedDifferentDirectory_HPP
#define test_directory_RenamedDifferentDirectory_HPP

#include "RenamedDifferentDirectory.d.hpp"

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

    void namespace_DifferentDirectory_destroy(RenamedDifferentDirectory* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const ns::capi::RenamedDifferentDirectory* ns::RenamedDifferentDirectory::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedDifferentDirectory*>(this);
}

inline ns::capi::RenamedDifferentDirectory* ns::RenamedDifferentDirectory::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedDifferentDirectory*>(this);
}

inline const ns::RenamedDifferentDirectory* ns::RenamedDifferentDirectory::FromFFI(const ns::capi::RenamedDifferentDirectory* ptr) {
  return reinterpret_cast<const ns::RenamedDifferentDirectory*>(ptr);
}

inline ns::RenamedDifferentDirectory* ns::RenamedDifferentDirectory::FromFFI(ns::capi::RenamedDifferentDirectory* ptr) {
  return reinterpret_cast<ns::RenamedDifferentDirectory*>(ptr);
}

inline void ns::RenamedDifferentDirectory::operator delete(void* ptr) {
  ns::capi::namespace_DifferentDirectory_destroy(reinterpret_cast<ns::capi::RenamedDifferentDirectory*>(ptr));
}


#endif // test_directory_RenamedDifferentDirectory_HPP
