#ifndef SOMELIB_mylib_Decimal_D_HPP
#define SOMELIB_mylib_Decimal_D_HPP

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
namespace capi { struct Decimal; }
class Decimal;
} // namespace mylib
} // namespace somelib



namespace somelib::mylib {
namespace capi {
    struct Decimal;
} // namespace capi
} // namespace

namespace somelib::mylib {
class Decimal {
public:

  /**
   * Test that method overloading works when methods are renamed to Python keywords.
   *
   * Problem: ICU4X has methods like from_int32(), from_int64(), from_uint32() that
   * are all renamed to "from" for a nice Python API. Since "from" is a Python keyword,
   * Diplomat escapes it to "from_". This caused a panic because the code only handled
   * Constructor overloading, not NamedConstructor.
   *
   * Expected Python API after fix:
   * d = Decimal.from_(42)       # calls from_int32
   * d = Decimal.from_(999999)   # calls from_int64
   *
   * Generated C++ should look like:
   * .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int32_t>(&mylib::Decimal::from))), "v"_a)
   * .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int64_t>(&mylib::Decimal::from))), "v"_a)
   * .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<uint32_t>(&mylib::Decimal::from))), "v"_a)
   */
  inline static std::unique_ptr<somelib::mylib::Decimal> from(int32_t _v);

  inline static std::unique_ptr<somelib::mylib::Decimal> from(int64_t _v);

  inline static std::unique_ptr<somelib::mylib::Decimal> from(uint32_t _v);

    inline const somelib::mylib::capi::Decimal* AsFFI() const;
    inline somelib::mylib::capi::Decimal* AsFFI();
    inline static const somelib::mylib::Decimal* FromFFI(const somelib::mylib::capi::Decimal* ptr);
    inline static somelib::mylib::Decimal* FromFFI(somelib::mylib::capi::Decimal* ptr);
    inline static void operator delete(void* ptr);
private:
    Decimal() = delete;
    Decimal(const somelib::mylib::Decimal&) = delete;
    Decimal(somelib::mylib::Decimal&&) noexcept = delete;
    Decimal operator=(const somelib::mylib::Decimal&) = delete;
    Decimal operator=(somelib::mylib::Decimal&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_mylib_Decimal_D_HPP
