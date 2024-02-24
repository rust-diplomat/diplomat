#ifndef Unnamespaced_D_HPP
#define Unnamespaced_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CPPRenamedAttrEnum.d.hpp"
#include "Unnamespaced.d.h"

namespace ns {
class AttrOpaque1Renamed;
class CPPRenamedAttrEnum;
}


class Unnamespaced {
public:

  inline static std::unique_ptr<Unnamespaced> make(ns::CPPRenamedAttrEnum _e);

  inline void use_namespaced(const ns::AttrOpaque1Renamed& _n) const;

  inline const capi::Unnamespaced* AsFFI() const;
  inline capi::Unnamespaced* AsFFI();
  inline static const Unnamespaced* FromFFI(const capi::Unnamespaced* ptr);
  inline static Unnamespaced* FromFFI(capi::Unnamespaced* ptr);
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
