#ifndef OptionString_HPP
#define OptionString_HPP

#include "OptionString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    OptionString* OptionString_new(const char* diplomat_str_data, size_t diplomat_str_len);
    
    typedef struct OptionString_write_result { bool is_ok;} OptionString_write_result;
    OptionString_write_result OptionString_write(const OptionString* self, DiplomatWrite* write);
    
    typedef struct OptionString_borrow_result {union {DiplomatStringView ok; }; bool is_ok;} OptionString_borrow_result;
    OptionString_borrow_result OptionString_borrow(const OptionString* self);
    
    
    void OptionString_destroy(OptionString* self);
    
    } // extern "C"
}

inline std::unique_ptr<OptionString> OptionString::new_(std::string_view diplomat_str) {
  auto result = capi::OptionString_new(diplomat_str.data(),
    diplomat_str.size());
  return std::unique_ptr<OptionString>(OptionString::FromFFI(result));
}

inline diplomat::result<std::string, std::monostate> OptionString::write() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::OptionString_write(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::monostate>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::optional<std::string_view> OptionString::borrow() const {
  auto result = capi::OptionString_borrow(this->AsFFI());
  return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline const capi::OptionString* OptionString::AsFFI() const {
  return reinterpret_cast<const capi::OptionString*>(this);
}

inline capi::OptionString* OptionString::AsFFI() {
  return reinterpret_cast<capi::OptionString*>(this);
}

inline const OptionString* OptionString::FromFFI(const capi::OptionString* ptr) {
  return reinterpret_cast<const OptionString*>(ptr);
}

inline OptionString* OptionString::FromFFI(capi::OptionString* ptr) {
  return reinterpret_cast<OptionString*>(ptr);
}

inline void OptionString::operator delete(void* ptr) {
  capi::OptionString_destroy(reinterpret_cast<capi::OptionString*>(ptr));
}


#endif // OptionString_HPP
