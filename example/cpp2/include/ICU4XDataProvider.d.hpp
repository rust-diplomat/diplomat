#ifndef ICU4XDataProvider_D_HPP
#define ICU4XDataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.d.h"


class ICU4XDataProvider {
public:

  inline static std::unique_ptr<ICU4XDataProvider> new_static();

  inline static diplomat::result<std::monostate, std::monostate> returns_result();

  inline const capi::ICU4XDataProvider* AsFFI() const;
  inline capi::ICU4XDataProvider* AsFFI();
  inline static const ICU4XDataProvider* FromFFI(const capi::ICU4XDataProvider* ptr);
  inline static ICU4XDataProvider* FromFFI(capi::ICU4XDataProvider* ptr);
  inline ~ICU4XDataProvider();
private:
  ICU4XDataProvider() = delete;
};


#endif // ICU4XDataProvider_D_HPP
