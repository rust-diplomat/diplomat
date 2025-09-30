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
} // namespace capi
} // namespace

namespace somelib {
class Float64VecError {
public:

  inline static std::unique_ptr<somelib::Float64VecError> new_(somelib::diplomat::span<const double> v);

  inline somelib::diplomat::result<double, std::monostate> operator[](size_t i) const;

    inline const somelib::capi::Float64VecError* AsFFI() const;
    inline somelib::capi::Float64VecError* AsFFI();
    inline static const somelib::Float64VecError* FromFFI(const somelib::capi::Float64VecError* ptr);
    inline static somelib::Float64VecError* FromFFI(somelib::capi::Float64VecError* ptr);
    inline static void operator delete(void* ptr);
private:
    Float64VecError() = delete;
    Float64VecError(const somelib::Float64VecError&) = delete;
    Float64VecError(somelib::Float64VecError&&) noexcept = delete;
    Float64VecError operator=(const somelib::Float64VecError&) = delete;
    Float64VecError operator=(somelib::Float64VecError&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Float64VecError_D_HPP
