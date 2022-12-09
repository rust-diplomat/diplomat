#ifndef RefListParameter_D_HPP
#define RefListParameter_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "RefListParameter.d.h"




class RefListParameter {
public:
  inline const capi::RefListParameter* AsFFI() const;
  inline capi::RefListParameter* AsFFI();

  inline ~RefListParameter();

private:
  RefListParameter() = delete;
};





#endif // RefListParameter_D_HPP
