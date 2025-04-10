#ifndef ns_RenamedMyIterator_HPP
#define ns_RenamedMyIterator_HPP

#include "RenamedMyIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    typedef struct namespace_MyIterator_next_result {union {uint8_t ok; }; bool is_ok;} namespace_MyIterator_next_result;
    namespace_MyIterator_next_result namespace_MyIterator_next(ns::capi::RenamedMyIterator* self);

    void namespace_MyIterator_destroy(RenamedMyIterator* self);

    } // extern "C"

} // namespace capi
} // namespace

inline std::optional<uint8_t> ns::RenamedMyIterator::next() {
  auto result = ns::capi::namespace_MyIterator_next(this->AsFFI());
  return result.is_ok ? std::optional<uint8_t>(result.ok) : std::nullopt;
}

inline const ns::capi::RenamedMyIterator* ns::RenamedMyIterator::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedMyIterator*>(this);
}

inline ns::capi::RenamedMyIterator* ns::RenamedMyIterator::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedMyIterator*>(this);
}

inline const ns::RenamedMyIterator* ns::RenamedMyIterator::FromFFI(const ns::capi::RenamedMyIterator* ptr) {
  return reinterpret_cast<const ns::RenamedMyIterator*>(ptr);
}

inline ns::RenamedMyIterator* ns::RenamedMyIterator::FromFFI(ns::capi::RenamedMyIterator* ptr) {
  return reinterpret_cast<ns::RenamedMyIterator*>(ptr);
}

inline void ns::RenamedMyIterator::operator delete(void* ptr) {
  ns::capi::namespace_MyIterator_destroy(reinterpret_cast<ns::capi::RenamedMyIterator*>(ptr));
}


#endif // ns_RenamedMyIterator_HPP
