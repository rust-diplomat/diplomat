#ifndef SOMELIB_ResultOpaque_D_HPP
#define SOMELIB_ResultOpaque_D_HPP

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
namespace capi { struct ResultOpaque; }
class ResultOpaque;
struct ErrorStruct;
class ErrorEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct ResultOpaque;
} // namespace capi
} // namespace

namespace somelib {
class ResultOpaque {
public:

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> new_(int32_t i);

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> new_failing_foo();

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> new_failing_bar();

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, std::monostate> new_failing_unit();

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorStruct> new_failing_struct(int32_t i);

  inline static somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ResultOpaque>> new_in_err(int32_t i);

  inline static somelib::diplomat::result<int32_t, std::monostate> new_int(int32_t i);

  inline static somelib::diplomat::result<somelib::ErrorEnum, std::unique_ptr<somelib::ResultOpaque>> new_in_enum_err(int32_t i);

  /**
   * When we take &str, the return type becomes a Result
   * Test that this interacts gracefully with returning a reference type
   */
  inline somelib::diplomat::result<somelib::ResultOpaque&, somelib::diplomat::Utf8Error> takes_str(std::string_view _v);

  inline void assert_integer(int32_t i) const;

    inline const somelib::capi::ResultOpaque* AsFFI() const;
    inline somelib::capi::ResultOpaque* AsFFI();
    inline static const somelib::ResultOpaque* FromFFI(const somelib::capi::ResultOpaque* ptr);
    inline static somelib::ResultOpaque* FromFFI(somelib::capi::ResultOpaque* ptr);
    inline static void operator delete(void* ptr);
private:
    ResultOpaque() = delete;
    ResultOpaque(const somelib::ResultOpaque&) = delete;
    ResultOpaque(somelib::ResultOpaque&&) noexcept = delete;
    ResultOpaque operator=(const somelib::ResultOpaque&) = delete;
    ResultOpaque operator=(somelib::ResultOpaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ResultOpaque_D_HPP
