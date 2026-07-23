#ifndef SOMELIB_mylib_MethodOverloading_D_HPP
#define SOMELIB_mylib_MethodOverloading_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace mylib {
namespace capi { struct MethodOverloading; }
class MethodOverloading;
} // namespace mylib
} // namespace somelib



namespace somelib::mylib {
namespace capi {
    struct MethodOverloading;
    extern "C" {
    void MethodOverloading_destroy(MethodOverloading* self);
    }
} // namespace capi
} // namespace

namespace somelib::mylib {
class MethodOverloading;
using MethodOverloadingRef = somelib::diplomat::Ref<MethodOverloading, const somelib::mylib::capi::MethodOverloading>;
using MethodOverloadingRefMut = somelib::diplomat::Ref<MethodOverloading, somelib::mylib::capi::MethodOverloading>;

class MethodOverloading : public somelib::diplomat::OpaquePointer<MethodOverloading, somelib::mylib::capi::MethodOverloading, somelib::mylib::capi::MethodOverloading_destroy> {
public:

  inline static somelib::mylib::MethodOverloading from(int32_t _v);

  inline static somelib::mylib::MethodOverloading from(int64_t _v);

  inline static somelib::mylib::MethodOverloading from(uint32_t _v);

};

} // namespace
#endif // SOMELIB_mylib_MethodOverloading_D_HPP
