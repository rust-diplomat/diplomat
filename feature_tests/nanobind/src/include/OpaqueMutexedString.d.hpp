#ifndef SOMELIB_OpaqueMutexedString_D_HPP
#define SOMELIB_OpaqueMutexedString_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct OpaqueMutexedString; }
class OpaqueMutexedString;
namespace capi { struct Utf16Wrap; }
class Utf16Wrap;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueMutexedString;
    extern "C" {
    void OpaqueMutexedString_destroy(OpaqueMutexedString* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueMutexedString;
using OpaqueMutexedStringRef = somelib::diplomat::Ref<OpaqueMutexedString, const somelib::capi::OpaqueMutexedString>;
using OpaqueMutexedStringRefMut = somelib::diplomat::Ref<OpaqueMutexedString, somelib::capi::OpaqueMutexedString>;

class OpaqueMutexedString : public somelib::diplomat::OpaquePointer<OpaqueMutexedString, somelib::capi::OpaqueMutexedString, somelib::capi::OpaqueMutexedString_destroy> {
public:

  inline static somelib::OpaqueMutexedString from_usize(size_t number);

  inline void change(size_t number) const;

  inline somelib::OpaqueMutexedStringRef borrow() const DIPLOMAT_LIFETIME_BOUND;

  inline static somelib::OpaqueMutexedStringRef borrow_other(const somelib::OpaqueMutexedString& other DIPLOMAT_LIFETIME_BOUND);

  inline somelib::OpaqueMutexedStringRef borrow_self_or_other(const somelib::OpaqueMutexedString& other DIPLOMAT_LIFETIME_BOUND) const DIPLOMAT_LIFETIME_BOUND;

  inline size_t get_len_and_add(size_t other) const;

  inline std::string_view dummy_str() const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::Utf16Wrap wrapper() const;

  inline uint16_t to_unsigned_from_unsigned(uint16_t input) const;

};

} // namespace
#endif // SOMELIB_OpaqueMutexedString_D_HPP
