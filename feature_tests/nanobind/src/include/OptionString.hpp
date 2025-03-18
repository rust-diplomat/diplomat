#ifndef OptionString_HPP
#define OptionString_HPP

#include "OptionString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::OptionString* OptionString_new(diplomat::capi::DiplomatStringView diplomat_str);
    
    typedef struct OptionString_write_result { bool is_ok;} OptionString_write_result;
    OptionString_write_result OptionString_write(const diplomat::capi::OptionString* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct OptionString_borrow_result {union {diplomat::capi::DiplomatStringView ok; }; bool is_ok;} OptionString_borrow_result;
    OptionString_borrow_result OptionString_borrow(const diplomat::capi::OptionString* self);
    
    
    void OptionString_destroy(OptionString* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<OptionString> OptionString::new_(std::string_view diplomat_str) {
  auto result = diplomat::capi::OptionString_new({diplomat_str.data(), diplomat_str.size()});
  return std::unique_ptr<OptionString>(OptionString::FromFFI(result));
}

inline diplomat::result<std::string, std::monostate> OptionString::write() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::OptionString_write(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::monostate>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::optional<std::string_view> OptionString::borrow() const {
  auto result = diplomat::capi::OptionString_borrow(this->AsFFI());
  return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline const diplomat::capi::OptionString* OptionString::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OptionString*>(this);
}

inline diplomat::capi::OptionString* OptionString::AsFFI() {
  return reinterpret_cast<diplomat::capi::OptionString*>(this);
}

inline const OptionString* OptionString::FromFFI(const diplomat::capi::OptionString* ptr) {
  return reinterpret_cast<const OptionString*>(ptr);
}

inline OptionString* OptionString::FromFFI(diplomat::capi::OptionString* ptr) {
  return reinterpret_cast<OptionString*>(ptr);
}

inline void OptionString::operator delete(void* ptr) {
  diplomat::capi::OptionString_destroy(reinterpret_cast<diplomat::capi::OptionString*>(ptr));
}


#endif // OptionString_HPP
