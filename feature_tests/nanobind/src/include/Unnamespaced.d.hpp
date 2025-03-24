#ifndef Unnamespaced_D_HPP
#define Unnamespaced_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
class RenamedAttrEnum;
}


namespace diplomat {
namespace capi {
    struct Unnamespaced;
} // namespace capi
} // namespace

class Unnamespaced {
public:

  inline static std::unique_ptr<Unnamespaced> make(ns::RenamedAttrEnum _e);

  inline void use_namespaced(const ns::AttrOpaque1Renamed& _n) const;

  inline const diplomat::capi::Unnamespaced* AsFFI() const;
  inline diplomat::capi::Unnamespaced* AsFFI();
  inline static const Unnamespaced* FromFFI(const diplomat::capi::Unnamespaced* ptr);
  inline static Unnamespaced* FromFFI(diplomat::capi::Unnamespaced* ptr);
  inline static void operator delete(void* ptr);
private:
  Unnamespaced() = delete;
  Unnamespaced(const Unnamespaced&) = delete;
  Unnamespaced(Unnamespaced&&) noexcept = delete;
  Unnamespaced operator=(const Unnamespaced&) = delete;
  Unnamespaced operator=(Unnamespaced&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Unnamespaced_D_HPP
