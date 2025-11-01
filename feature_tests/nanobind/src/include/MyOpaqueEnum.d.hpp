#ifndef SOMELIB_MyOpaqueEnum_D_HPP
#define SOMELIB_MyOpaqueEnum_D_HPP

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
namespace capi { struct MyOpaqueEnum; }
class MyOpaqueEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MyOpaqueEnum;
} // namespace capi
} // namespace

namespace somelib {
class MyOpaqueEnum {
public:

  inline static std::unique_ptr<somelib::MyOpaqueEnum> new_();

  inline std::string to_string() const;
  template<typename W>
  inline void to_string_write(W& writeable_output) const;

    inline const somelib::capi::MyOpaqueEnum* AsFFI() const;
    inline somelib::capi::MyOpaqueEnum* AsFFI();
    inline static const somelib::MyOpaqueEnum* FromFFI(const somelib::capi::MyOpaqueEnum* ptr);
    inline static somelib::MyOpaqueEnum* FromFFI(somelib::capi::MyOpaqueEnum* ptr);
    inline static void operator delete(void* ptr);
private:
    MyOpaqueEnum() = delete;
    MyOpaqueEnum(const somelib::MyOpaqueEnum&) = delete;
    MyOpaqueEnum(somelib::MyOpaqueEnum&&) noexcept = delete;
    MyOpaqueEnum operator=(const somelib::MyOpaqueEnum&) = delete;
    MyOpaqueEnum operator=(somelib::MyOpaqueEnum&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_MyOpaqueEnum_D_HPP
