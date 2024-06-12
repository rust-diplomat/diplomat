#ifndef OpaqueMutexedString_HPP
#define OpaqueMutexedString_HPP

#include "OpaqueMutexedString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Utf16Wrap.hpp"


namespace capi {
    extern "C" {
    
    OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);
    
    void OpaqueMutexedString_change(const OpaqueMutexedString* self, size_t number);
    
    const OpaqueMutexedString* OpaqueMutexedString_borrow(const OpaqueMutexedString* self);
    
    const OpaqueMutexedString* OpaqueMutexedString_borrow_other(const OpaqueMutexedString* other);
    
    const OpaqueMutexedString* OpaqueMutexedString_borrow_self_or_other(const OpaqueMutexedString* self, const OpaqueMutexedString* other);
    
    size_t OpaqueMutexedString_get_len_and_add(const OpaqueMutexedString* self, size_t other);
    
    DiplomatStringView OpaqueMutexedString_dummy_str(const OpaqueMutexedString* self);
    
    Utf16Wrap* OpaqueMutexedString_wrapper(const OpaqueMutexedString* self);
    
    
    void OpaqueMutexedString_destroy(OpaqueMutexedString* self);
    
    } // extern "C"
}

inline std::unique_ptr<OpaqueMutexedString> OpaqueMutexedString::from_usize(size_t number) {
  auto result = capi::OpaqueMutexedString_from_usize(number);
  return std::unique_ptr<OpaqueMutexedString>(OpaqueMutexedString::FromFFI(result));
}

inline void OpaqueMutexedString::change(size_t number) const {
  capi::OpaqueMutexedString_change(this->AsFFI(),
    number);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow() const {
  auto result = capi::OpaqueMutexedString_borrow(this->AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow_other(const OpaqueMutexedString& other) {
  auto result = capi::OpaqueMutexedString_borrow_other(other.AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline const OpaqueMutexedString& OpaqueMutexedString::borrow_self_or_other(const OpaqueMutexedString& other) const {
  auto result = capi::OpaqueMutexedString_borrow_self_or_other(this->AsFFI(),
    other.AsFFI());
  return *OpaqueMutexedString::FromFFI(result);
}

inline size_t OpaqueMutexedString::get_len_and_add(size_t other) const {
  auto result = capi::OpaqueMutexedString_get_len_and_add(this->AsFFI(),
    other);
  return result;
}

inline std::string_view OpaqueMutexedString::dummy_str() const {
  auto result = capi::OpaqueMutexedString_dummy_str(this->AsFFI());
  return std::string_view(result.data, result.len);
}

inline std::unique_ptr<Utf16Wrap> OpaqueMutexedString::wrapper() const {
  auto result = capi::OpaqueMutexedString_wrapper(this->AsFFI());
  return std::unique_ptr<Utf16Wrap>(Utf16Wrap::FromFFI(result));
}

inline const capi::OpaqueMutexedString* OpaqueMutexedString::AsFFI() const {
  return reinterpret_cast<const capi::OpaqueMutexedString*>(this);
}

inline capi::OpaqueMutexedString* OpaqueMutexedString::AsFFI() {
  return reinterpret_cast<capi::OpaqueMutexedString*>(this);
}

inline const OpaqueMutexedString* OpaqueMutexedString::FromFFI(const capi::OpaqueMutexedString* ptr) {
  return reinterpret_cast<const OpaqueMutexedString*>(ptr);
}

inline OpaqueMutexedString* OpaqueMutexedString::FromFFI(capi::OpaqueMutexedString* ptr) {
  return reinterpret_cast<OpaqueMutexedString*>(ptr);
}

inline void OpaqueMutexedString::operator delete(void* ptr) {
  capi::OpaqueMutexedString_destroy(reinterpret_cast<capi::OpaqueMutexedString*>(ptr));
}


#endif // OpaqueMutexedString_HPP
