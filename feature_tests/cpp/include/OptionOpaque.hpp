#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP

#include "OptionOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "OptionStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::OptionOpaque* OptionOpaque_new(int32_t i);
    
    diplomat::capi::OptionOpaque* OptionOpaque_new_none(void);
    
    typedef struct OptionOpaque_returns_result {union {diplomat::capi::OptionStruct ok; }; bool is_ok;} OptionOpaque_returns_result;
    OptionOpaque_returns_result OptionOpaque_returns(void);
    
    typedef struct OptionOpaque_option_isize_result {union {intptr_t ok; }; bool is_ok;} OptionOpaque_option_isize_result;
    OptionOpaque_option_isize_result OptionOpaque_option_isize(const diplomat::capi::OptionOpaque* self);
    
    typedef struct OptionOpaque_option_usize_result {union {size_t ok; }; bool is_ok;} OptionOpaque_option_usize_result;
    OptionOpaque_option_usize_result OptionOpaque_option_usize(const diplomat::capi::OptionOpaque* self);
    
    typedef struct OptionOpaque_option_i32_result {union {int32_t ok; }; bool is_ok;} OptionOpaque_option_i32_result;
    OptionOpaque_option_i32_result OptionOpaque_option_i32(const diplomat::capi::OptionOpaque* self);
    
    typedef struct OptionOpaque_option_u32_result {union {uint32_t ok; }; bool is_ok;} OptionOpaque_option_u32_result;
    OptionOpaque_option_u32_result OptionOpaque_option_u32(const diplomat::capi::OptionOpaque* self);
    
    diplomat::capi::OptionStruct OptionOpaque_new_struct(void);
    
    diplomat::capi::OptionStruct OptionOpaque_new_struct_nones(void);
    
    void OptionOpaque_assert_integer(const diplomat::capi::OptionOpaque* self, int32_t i);
    
    bool OptionOpaque_option_opaque_argument(const diplomat::capi::OptionOpaque* arg);
    
    
    void OptionOpaque_destroy(OptionOpaque* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_(int32_t i) {
  auto result = diplomat::capi::OptionOpaque_new(i);
  return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_none() {
  auto result = diplomat::capi::OptionOpaque_new_none();
  return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline std::optional<OptionStruct> OptionOpaque::returns() {
  auto result = diplomat::capi::OptionOpaque_returns();
  return result.is_ok ? std::optional<OptionStruct>(OptionStruct::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<intptr_t> OptionOpaque::option_isize() const {
  auto result = diplomat::capi::OptionOpaque_option_isize(this->AsFFI());
  return result.is_ok ? std::optional<intptr_t>(result.ok) : std::nullopt;
}

inline std::optional<size_t> OptionOpaque::option_usize() const {
  auto result = diplomat::capi::OptionOpaque_option_usize(this->AsFFI());
  return result.is_ok ? std::optional<size_t>(result.ok) : std::nullopt;
}

inline std::optional<int32_t> OptionOpaque::option_i32() const {
  auto result = diplomat::capi::OptionOpaque_option_i32(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<uint32_t> OptionOpaque::option_u32() const {
  auto result = diplomat::capi::OptionOpaque_option_u32(this->AsFFI());
  return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline OptionStruct OptionOpaque::new_struct() {
  auto result = diplomat::capi::OptionOpaque_new_struct();
  return OptionStruct::FromFFI(result);
}

inline OptionStruct OptionOpaque::new_struct_nones() {
  auto result = diplomat::capi::OptionOpaque_new_struct_nones();
  return OptionStruct::FromFFI(result);
}

inline void OptionOpaque::assert_integer(int32_t i) const {
  diplomat::capi::OptionOpaque_assert_integer(this->AsFFI(),
    i);
}

inline bool OptionOpaque::option_opaque_argument(const OptionOpaque* arg) {
  auto result = diplomat::capi::OptionOpaque_option_opaque_argument(arg ? arg->AsFFI() : nullptr);
  return result;
}

inline const diplomat::capi::OptionOpaque* OptionOpaque::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OptionOpaque*>(this);
}

inline diplomat::capi::OptionOpaque* OptionOpaque::AsFFI() {
  return reinterpret_cast<diplomat::capi::OptionOpaque*>(this);
}

inline const OptionOpaque* OptionOpaque::FromFFI(const diplomat::capi::OptionOpaque* ptr) {
  return reinterpret_cast<const OptionOpaque*>(ptr);
}

inline OptionOpaque* OptionOpaque::FromFFI(diplomat::capi::OptionOpaque* ptr) {
  return reinterpret_cast<OptionOpaque*>(ptr);
}

inline void OptionOpaque::operator delete(void* ptr) {
  diplomat::capi::OptionOpaque_destroy(reinterpret_cast<diplomat::capi::OptionOpaque*>(ptr));
}


#endif // OptionOpaque_HPP
