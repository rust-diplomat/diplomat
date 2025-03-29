#ifndef MyOpaqueEnum_D_HPP
#define MyOpaqueEnum_D_HPP

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
    struct MyOpaqueEnum;
} // namespace capi
} // namespace

class MyOpaqueEnum {
public:

  inline static std::unique_ptr<MyOpaqueEnum> new_();

  inline std::string to_string() const;

  inline const diplomat::capi::MyOpaqueEnum* AsFFI() const;
  inline diplomat::capi::MyOpaqueEnum* AsFFI();
  inline static const MyOpaqueEnum* FromFFI(const diplomat::capi::MyOpaqueEnum* ptr);
  inline static MyOpaqueEnum* FromFFI(diplomat::capi::MyOpaqueEnum* ptr);
  inline static void operator delete(void* ptr);
private:
  MyOpaqueEnum() = delete;
  MyOpaqueEnum(const MyOpaqueEnum&) = delete;
  MyOpaqueEnum(MyOpaqueEnum&&) noexcept = delete;
  MyOpaqueEnum operator=(const MyOpaqueEnum&) = delete;
  MyOpaqueEnum operator=(MyOpaqueEnum&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MyOpaqueEnum_D_HPP
