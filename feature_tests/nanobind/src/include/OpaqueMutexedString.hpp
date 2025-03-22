#ifndef OpaqueMutexedString_HPP
#define OpaqueMutexedString_HPP

#include "OpaqueMutexedString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Utf16Wrap.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);
    
    void OpaqueMutexedString_change(const diplomat::capi::OpaqueMutexedString* self, size_t number);
    
    const diplomat::capi::OpaqueMutexedString* OpaqueMutexedString_borrow(const diplomat::capi::OpaqueMutexedString* self);
    
    const diplomat::capi::OpaqueMutexedString* OpaqueMutexedString_borrow_other(const diplomat::capi::OpaqueMutexedString* other);
    
    const diplomat::capi::OpaqueMutexedString* OpaqueMutexedString_borrow_self_or_other(const diplomat::capi::OpaqueMutexedString* self, const diplomat::capi::OpaqueMutexedString* other);
    
    size_t OpaqueMutexedString_get_len_and_add(const diplomat::capi::OpaqueMutexedString* self, size_t other);
    
    diplomat::capi::DiplomatStringView OpaqueMutexedString_dummy_str(const diplomat::capi::OpaqueMutexedString* self);
    
    diplomat::capi::Utf16Wrap* OpaqueMutexedString_wrapper(const diplomat::capi::OpaqueMutexedString* self);
    
    uint16_t OpaqueMutexedString_to_unsigned_from_unsigned(const diplomat::capi::OpaqueMutexedString* self, uint16_t input);
    
    
    void OpaqueMutexedString_destroy(OpaqueMutexedString* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<OpaqueMutexedString> OpaqueMutexedString::from_usize(size_t number) {
  auto result = diplomat::capi::OpaqueMutexedString_from_usize(number);
  return std::unique_ptr<OpaqueMutexedString>(OpaqueMutexedString::FromFFI(result));
}

inline void OpaqueMutexedString::change(size_t number) const {
  diplomat::capi::OpaqueMutexedString_change(this->AsFFI(),
    number);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow() const {
  auto result = diplomat::capi::OpaqueMutexedString_borrow(this->AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow_other(const OpaqueMutexedString& other) {
  auto result = diplomat::capi::OpaqueMutexedString_borrow_other(other.AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow_self_or_other(const OpaqueMutexedString& other) const {
  auto result = diplomat::capi::OpaqueMutexedString_borrow_self_or_other(this->AsFFI(),
    other.AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline size_t OpaqueMutexedString::get_len_and_add(size_t other) const {
  auto result = diplomat::capi::OpaqueMutexedString_get_len_and_add(this->AsFFI(),
    other);
  return result;
}

inline std::string_view OpaqueMutexedString::dummy_str() const {
  auto result = diplomat::capi::OpaqueMutexedString_dummy_str(this->AsFFI());
  return std::string_view(result.data, result.len);
}

inline std::unique_ptr<Utf16Wrap> OpaqueMutexedString::wrapper() const {
  auto result = diplomat::capi::OpaqueMutexedString_wrapper(this->AsFFI());
  return std::unique_ptr<Utf16Wrap>(Utf16Wrap::FromFFI(result));
}

inline uint16_t OpaqueMutexedString::to_unsigned_from_unsigned(uint16_t input) const {
  auto result = diplomat::capi::OpaqueMutexedString_to_unsigned_from_unsigned(this->AsFFI(),
    input);
  return result;
}

inline const diplomat::capi::OpaqueMutexedString* OpaqueMutexedString::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OpaqueMutexedString*>(this);
}

inline diplomat::capi::OpaqueMutexedString* OpaqueMutexedString::AsFFI() {
  return reinterpret_cast<diplomat::capi::OpaqueMutexedString*>(this);
}

inline const OpaqueMutexedString* OpaqueMutexedString::FromFFI(const diplomat::capi::OpaqueMutexedString* ptr) {
  return reinterpret_cast<const OpaqueMutexedString*>(ptr);
}

inline OpaqueMutexedString* OpaqueMutexedString::FromFFI(diplomat::capi::OpaqueMutexedString* ptr) {
  return reinterpret_cast<OpaqueMutexedString*>(ptr);
}

inline void OpaqueMutexedString::operator delete(void* ptr) {
  diplomat::capi::OpaqueMutexedString_destroy(reinterpret_cast<diplomat::capi::OpaqueMutexedString*>(ptr));
}


#endif // OpaqueMutexedString_HPP
