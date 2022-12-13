#ifndef RefListParameter_D_HPP
#define RefListParameter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "RefListParameter.d.h"


class RefListParameter {
public:

  inline const capi::RefListParameter* AsFFI() const;
  inline capi::RefListParameter* AsFFI();
  inline static const RefListParameter* FromFFI(const capi::RefListParameter* ptr);
  inline static RefListParameter* FromFFI(capi::RefListParameter* ptr);
  inline ~RefListParameter();
private:
  RefListParameter() = delete;
};


#endif // RefListParameter_D_HPP
