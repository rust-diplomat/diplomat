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
} // namespace capi
} // namespace

namespace somelib::mylib {
class MethodOverloading {
public:

  inline static std::unique_ptr<somelib::mylib::MethodOverloading> from(int32_t _v);

  inline static std::unique_ptr<somelib::mylib::MethodOverloading> from(int64_t _v);

  inline static std::unique_ptr<somelib::mylib::MethodOverloading> from(uint32_t _v);

    inline const somelib::mylib::capi::MethodOverloading* AsFFI() const;
    inline somelib::mylib::capi::MethodOverloading* AsFFI();
    inline static const somelib::mylib::MethodOverloading* FromFFI(const somelib::mylib::capi::MethodOverloading* ptr);
    inline static somelib::mylib::MethodOverloading* FromFFI(somelib::mylib::capi::MethodOverloading* ptr);
    inline static void operator delete(void* ptr);
private:
    MethodOverloading() = delete;
    MethodOverloading(const somelib::mylib::MethodOverloading&) = delete;
    MethodOverloading(somelib::mylib::MethodOverloading&&) noexcept = delete;
    MethodOverloading operator=(const somelib::mylib::MethodOverloading&) = delete;
    MethodOverloading operator=(somelib::mylib::MethodOverloading&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_mylib_MethodOverloading_D_HPP
