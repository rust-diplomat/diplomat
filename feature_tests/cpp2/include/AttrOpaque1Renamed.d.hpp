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

  inline static std::unique_ptr<AttrOpaque1Renamed> totally_not_new();

  inline uint8_t method_renamed() const;

  inline uint8_t abirenamed() const;

  inline const capi::AttrOpaque1* AsFFI() const;
  inline capi::AttrOpaque1* AsFFI();
  inline static const AttrOpaque1Renamed* FromFFI(const capi::AttrOpaque1* ptr);
  inline static AttrOpaque1Renamed* FromFFI(capi::AttrOpaque1* ptr);
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
