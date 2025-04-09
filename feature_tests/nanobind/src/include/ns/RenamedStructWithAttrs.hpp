#ifndef ns_RenamedStructWithAttrs_HPP
#define ns_RenamedStructWithAttrs_HPP

#include "RenamedStructWithAttrs.d.hpp"

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
    typedef struct namespace_StructWithAttrs_new_fallible_result {union {ns::capi::RenamedStructWithAttrs ok; }; bool is_ok;} namespace_StructWithAttrs_new_fallible_result;
    namespace_StructWithAttrs_new_fallible_result namespace_StructWithAttrs_new_fallible(bool a, uint32_t b);
    uint32_t namespace_StructWithAttrs_c(ns::capi::RenamedStructWithAttrs self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<ns::RenamedStructWithAttrs, std::monostate> ns::RenamedStructWithAttrs::new_fallible(bool a, uint32_t b) {
  auto result = ns::capi::namespace_StructWithAttrs_new_fallible(a,
    b);
  return result.is_ok ? diplomat::result<ns::RenamedStructWithAttrs, std::monostate>(diplomat::Ok<ns::RenamedStructWithAttrs>(ns::RenamedStructWithAttrs::FromFFI(result.ok))) : diplomat::result<ns::RenamedStructWithAttrs, std::monostate>(diplomat::Err<std::monostate>());
}

inline uint32_t ns::RenamedStructWithAttrs::c() const {
  auto result = ns::capi::namespace_StructWithAttrs_c(this->AsFFI());
  return result;
}


inline ns::capi::RenamedStructWithAttrs ns::RenamedStructWithAttrs::AsFFI() const {
  return ns::capi::RenamedStructWithAttrs {
    /* .a = */ a,
    /* .b = */ b,
  };
}

inline ns::RenamedStructWithAttrs ns::RenamedStructWithAttrs::FromFFI(ns::capi::RenamedStructWithAttrs c_struct) {
  return ns::RenamedStructWithAttrs {
    /* .a = */ c_struct.a,
    /* .b = */ c_struct.b,
  };
}


#endif // ns_RenamedStructWithAttrs_HPP
