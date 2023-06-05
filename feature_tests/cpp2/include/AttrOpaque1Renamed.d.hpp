#ifndef AttrOpaque1Renamed_D_HPP
#define AttrOpaque1Renamed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque1.d.h"


class AttrOpaque1Renamed {
public:

  inline void method_renamed() const;

  inline const capi::AttrOpaque1Renamed* AsFFI() const;
  inline capi::AttrOpaque1Renamed* AsFFI();
  inline static const AttrOpaque1Renamed* FromFFI(const capi::AttrOpaque1Renamed* ptr);
  inline static AttrOpaque1Renamed* FromFFI(capi::AttrOpaque1Renamed* ptr);
  inline static void operator delete(void* ptr);
private:
  AttrOpaque1Renamed() = delete;
  AttrOpaque1Renamed(const AttrOpaque1Renamed&) = delete;
  AttrOpaque1Renamed(AttrOpaque1Renamed&&) noexcept = delete;
  AttrOpaque1Renamed operator=(const AttrOpaque1Renamed&) = delete;
  AttrOpaque1Renamed operator=(AttrOpaque1Renamed&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // AttrOpaque1Renamed_D_HPP
