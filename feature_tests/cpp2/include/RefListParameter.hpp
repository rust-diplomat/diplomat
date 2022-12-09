#ifndef RefListParameter_HPP
#define RefListParameter_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "RefListParameter.d.hpp"
#include "RefListParameter.h"




inline const capi::RefListParameter* RefListParameter::AsFFI() const {
  return reinterpret_cast<const capi::RefListParameter*>(this);
}
inline capi::RefListParameter* RefListParameter::AsFFI() {
  return reinterpret_cast<capi::RefListParameter*>(this);
}
inline RefListParameter::~RefListParameter() {
  capi::RefListParameter_destroy(AsFFI());
}


#endif // RefListParameter_HPP
