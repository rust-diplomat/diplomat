#ifndef ns_RenamedVectorTest_HPP
#define ns_RenamedVectorTest_HPP

#include "RenamedVectorTest.d.hpp"

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

    ns::capi::RenamedVectorTest* namespace_VectorTest_new(void);

    size_t namespace_VectorTest_len(const ns::capi::RenamedVectorTest* self);

    typedef struct namespace_VectorTest_get_result {union {double ok; }; bool is_ok;} namespace_VectorTest_get_result;
    namespace_VectorTest_get_result namespace_VectorTest_get(const ns::capi::RenamedVectorTest* self, size_t idx);

    void namespace_VectorTest_push(ns::capi::RenamedVectorTest* self, double val);

    void namespace_VectorTest_destroy(RenamedVectorTest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedVectorTest> ns::RenamedVectorTest::new_() {
  auto result = ns::capi::namespace_VectorTest_new();
  return std::unique_ptr<ns::RenamedVectorTest>(ns::RenamedVectorTest::FromFFI(result));
}

inline size_t ns::RenamedVectorTest::len() const {
  auto result = ns::capi::namespace_VectorTest_len(this->AsFFI());
  return result;
}

inline std::optional<double> ns::RenamedVectorTest::operator[](size_t idx) const {
  auto result = ns::capi::namespace_VectorTest_get(this->AsFFI(),
    idx);
  return result.is_ok ? std::optional<double>(result.ok) : std::nullopt;
}

inline void ns::RenamedVectorTest::push(double val) {
  ns::capi::namespace_VectorTest_push(this->AsFFI(),
    val);
}

inline const ns::capi::RenamedVectorTest* ns::RenamedVectorTest::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedVectorTest*>(this);
}

inline ns::capi::RenamedVectorTest* ns::RenamedVectorTest::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedVectorTest*>(this);
}

inline const ns::RenamedVectorTest* ns::RenamedVectorTest::FromFFI(const ns::capi::RenamedVectorTest* ptr) {
  return reinterpret_cast<const ns::RenamedVectorTest*>(ptr);
}

inline ns::RenamedVectorTest* ns::RenamedVectorTest::FromFFI(ns::capi::RenamedVectorTest* ptr) {
  return reinterpret_cast<ns::RenamedVectorTest*>(ptr);
}

inline void ns::RenamedVectorTest::operator delete(void* ptr) {
  ns::capi::namespace_VectorTest_destroy(reinterpret_cast<ns::capi::RenamedVectorTest*>(ptr));
}


#endif // ns_RenamedVectorTest_HPP
