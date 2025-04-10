#ifndef ns_RenamedMyIterable_HPP
#define ns_RenamedMyIterable_HPP

#include "RenamedMyIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "RenamedMyIterator.hpp"


namespace ns {
namespace capi {
    extern "C" {
    ns::capi::RenamedMyIterable* namespace_MyIterable_new(diplomat::capi::DiplomatU8View x);
    ns::capi::RenamedMyIterator* namespace_MyIterable_iter(const ns::capi::RenamedMyIterable* self);

    void namespace_MyIterable_destroy(RenamedMyIterable* self);

    } // extern "C"

} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedMyIterable> ns::RenamedMyIterable::new_(diplomat::span<const uint8_t> x) {
  auto result = ns::capi::namespace_MyIterable_new({x.data(), x.size()});
  return std::unique_ptr<ns::RenamedMyIterable>(ns::RenamedMyIterable::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedMyIterator> ns::RenamedMyIterable::iter() const {
  auto result = ns::capi::namespace_MyIterable_iter(this->AsFFI());
  return std::unique_ptr<ns::RenamedMyIterator>(ns::RenamedMyIterator::FromFFI(result));
}

inline diplomat::next_to_iter_helper<ns::RenamedMyIterator>ns::RenamedMyIterable::begin() const {
  return iter();
}

inline const ns::capi::RenamedMyIterable* ns::RenamedMyIterable::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedMyIterable*>(this);
}

inline ns::capi::RenamedMyIterable* ns::RenamedMyIterable::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedMyIterable*>(this);
}

inline const ns::RenamedMyIterable* ns::RenamedMyIterable::FromFFI(const ns::capi::RenamedMyIterable* ptr) {
  return reinterpret_cast<const ns::RenamedMyIterable*>(ptr);
}

inline ns::RenamedMyIterable* ns::RenamedMyIterable::FromFFI(ns::capi::RenamedMyIterable* ptr) {
  return reinterpret_cast<ns::RenamedMyIterable*>(ptr);
}

inline void ns::RenamedMyIterable::operator delete(void* ptr) {
  ns::capi::namespace_MyIterable_destroy(reinterpret_cast<ns::capi::RenamedMyIterable*>(ptr));
}


#endif // ns_RenamedMyIterable_HPP
