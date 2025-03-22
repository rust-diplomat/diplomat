#ifndef MyOpaqueEnum_HPP
#define MyOpaqueEnum_HPP

#include "MyOpaqueEnum.d.hpp"

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
    
    diplomat::capi::MyOpaqueEnum* MyOpaqueEnum_new(void);
    
    void MyOpaqueEnum_to_string(const diplomat::capi::MyOpaqueEnum* self, diplomat::capi::DiplomatWrite* write);
    
    
    void MyOpaqueEnum_destroy(MyOpaqueEnum* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<MyOpaqueEnum> MyOpaqueEnum::new_() {
  auto result = diplomat::capi::MyOpaqueEnum_new();
  return std::unique_ptr<MyOpaqueEnum>(MyOpaqueEnum::FromFFI(result));
}

inline std::string MyOpaqueEnum::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::MyOpaqueEnum_to_string(this->AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::MyOpaqueEnum* MyOpaqueEnum::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MyOpaqueEnum*>(this);
}

inline diplomat::capi::MyOpaqueEnum* MyOpaqueEnum::AsFFI() {
  return reinterpret_cast<diplomat::capi::MyOpaqueEnum*>(this);
}

inline const MyOpaqueEnum* MyOpaqueEnum::FromFFI(const diplomat::capi::MyOpaqueEnum* ptr) {
  return reinterpret_cast<const MyOpaqueEnum*>(ptr);
}

inline MyOpaqueEnum* MyOpaqueEnum::FromFFI(diplomat::capi::MyOpaqueEnum* ptr) {
  return reinterpret_cast<MyOpaqueEnum*>(ptr);
}

inline void MyOpaqueEnum::operator delete(void* ptr) {
  diplomat::capi::MyOpaqueEnum_destroy(reinterpret_cast<diplomat::capi::MyOpaqueEnum*>(ptr));
}


#endif // MyOpaqueEnum_HPP
