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
    extern "C" {
    void MyOpaqueEnum_destroy(MyOpaqueEnum* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class MyOpaqueEnum;
using MyOpaqueEnumRef = somelib::diplomat::Ref<MyOpaqueEnum, const somelib::capi::MyOpaqueEnum>;
using MyOpaqueEnumRefMut = somelib::diplomat::Ref<MyOpaqueEnum, somelib::capi::MyOpaqueEnum>;

class MyOpaqueEnum : public somelib::diplomat::OpaquePointer<MyOpaqueEnum, somelib::capi::MyOpaqueEnum, somelib::capi::MyOpaqueEnum_destroy> {
public:

  inline static somelib::MyOpaqueEnum new_();

  inline std::string to_string() const;
  template<typename W>
  inline void to_string_write(W& writeable_output) const;

};

} // namespace
#endif // SOMELIB_MyOpaqueEnum_D_HPP
