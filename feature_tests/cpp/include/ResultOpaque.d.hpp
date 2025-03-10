#ifndef ResultOpaque_D_HPP
#define ResultOpaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

struct ErrorStruct;
class ErrorEnum;


namespace diplomat {
namespace capi {
    struct ResultOpaque;
} // namespace capi
} // namespace

class ResultOpaque {
public:

  inline static diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> new_(int32_t i);

  inline static diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_foo();

  inline static diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_bar();

  inline static diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate> new_failing_unit();

  inline static diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct> new_failing_struct(int32_t i);

  inline static diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>> new_in_err(int32_t i);

  inline static diplomat::result<int32_t, std::monostate> new_int(int32_t i);

  inline static diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>> new_in_enum_err(int32_t i);

  inline void assert_integer(int32_t i) const;

  inline const diplomat::capi::ResultOpaque* AsFFI() const;
  inline diplomat::capi::ResultOpaque* AsFFI();
  inline static const ResultOpaque* FromFFI(const diplomat::capi::ResultOpaque* ptr);
  inline static ResultOpaque* FromFFI(diplomat::capi::ResultOpaque* ptr);
  inline static void operator delete(void* ptr);
private:
  ResultOpaque() = delete;
  ResultOpaque(const ResultOpaque&) = delete;
  ResultOpaque(ResultOpaque&&) noexcept = delete;
  ResultOpaque operator=(const ResultOpaque&) = delete;
  ResultOpaque operator=(ResultOpaque&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ResultOpaque_D_HPP
