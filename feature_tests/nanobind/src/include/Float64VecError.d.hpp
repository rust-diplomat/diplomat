#ifndef SOMELIB_Float64VecError_D_HPP
#define SOMELIB_Float64VecError_D_HPP

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
namespace capi { struct Float64VecError; }
class Float64VecError;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Float64VecError;
    extern "C" {
    void Float64VecError_destroy(Float64VecError* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Float64VecError;
using Float64VecErrorRef = somelib::diplomat::Ref<Float64VecError, const somelib::capi::Float64VecError>;
using Float64VecErrorRefMut = somelib::diplomat::Ref<Float64VecError, somelib::capi::Float64VecError>;

class Float64VecError : public somelib::diplomat::OpaquePointer<Float64VecError, somelib::capi::Float64VecError, somelib::capi::Float64VecError_destroy> {
public:

  inline static somelib::Float64VecError new_(somelib::diplomat::span<const double> v);

  inline somelib::diplomat::result<double, std::monostate> operator[](size_t i) const;

};

} // namespace
#endif // SOMELIB_Float64VecError_D_HPP
