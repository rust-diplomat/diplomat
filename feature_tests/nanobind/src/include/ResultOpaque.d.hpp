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
    extern "C" {
    void ResultOpaque_destroy(ResultOpaque* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class ResultOpaque;
using ResultOpaqueRef = somelib::diplomat::Ref<ResultOpaque, const somelib::capi::ResultOpaque>;
using ResultOpaqueRefMut = somelib::diplomat::Ref<ResultOpaque, somelib::capi::ResultOpaque>;

class ResultOpaque : public somelib::diplomat::OpaquePointer<ResultOpaque, somelib::capi::ResultOpaque, somelib::capi::ResultOpaque_destroy> {
public:

  inline static somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> new_(int32_t i);

  inline static somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> new_failing_foo();

  inline static somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> new_failing_bar();

  inline static somelib::diplomat::result<somelib::ResultOpaque, std::monostate> new_failing_unit();

  inline static somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorStruct> new_failing_struct(int32_t i);

  inline static somelib::diplomat::result<std::monostate, somelib::ResultOpaque> new_in_err(int32_t i);

  inline static somelib::diplomat::result<int32_t, std::monostate> new_int(int32_t i);

  inline static somelib::diplomat::result<somelib::ErrorEnum, somelib::ResultOpaque> new_in_enum_err(int32_t i);

  inline somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef> give_self() const DIPLOMAT_LIFETIME_BOUND;

  /**
   * When we take &str, the return type becomes a Result
   * Test that this interacts gracefully with returning a reference type
   */
  inline somelib::diplomat::result<somelib::ResultOpaqueRefMut, somelib::diplomat::Utf8Error> takes_str(std::string_view _v) DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::result<std::string, somelib::ResultOpaqueRef> stringify_error() const DIPLOMAT_LIFETIME_BOUND;
  template<typename W>
  inline somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef> stringify_error_write(W& writeable_output) const DIPLOMAT_LIFETIME_BOUND;

  inline void assert_integer(int32_t i) const;

};

} // namespace
#endif // SOMELIB_ResultOpaque_D_HPP
