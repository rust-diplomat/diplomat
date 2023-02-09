#ifndef Bar_D_HPP
#define Bar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Bar.d.h"


class Bar {
public:

  inline const capi::Bar* AsFFI() const;
  inline capi::Bar* AsFFI();
  inline static const Bar* FromFFI(const capi::Bar* ptr);
  inline static Bar* FromFFI(capi::Bar* ptr);
  inline static void operator delete(void* ptr);
private:
  Bar() = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Bar_D_HPP
