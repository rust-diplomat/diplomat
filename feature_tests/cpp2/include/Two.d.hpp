#ifndef Two_D_HPP
#define Two_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Two.d.h"


class Two {
public:

  inline const capi::Two* AsFFI() const;
  inline capi::Two* AsFFI();
  inline static const Two* FromFFI(const capi::Two* ptr);
  inline static Two* FromFFI(capi::Two* ptr);
  inline static void operator delete(void* ptr);
private:
  Two() = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Two_D_HPP
