#ifndef AttrOpaque1Renamed_D_HPP
#define AttrOpaque1Renamed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct Unnamespaced Unnamespaced; }
class Unnamespaced;
namespace ns {
namespace capi {typedef struct AttrOpaque1Renamed AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
class CPPRenamedAttrEnum;
}


namespace ns {
namespace capi {
    typedef struct AttrOpaque1Renamed AttrOpaque1Renamed;
} // namespace capi
} // namespace

namespace ns {
class AttrOpaque1Renamed {
public:

  inline static std::unique_ptr<ns::AttrOpaque1Renamed> totally_not_new();

  inline uint8_t method_renamed() const;

  inline uint8_t abirenamed() const;

  inline void use_unnamespaced(const Unnamespaced& _un) const;

  inline void use_namespaced(ns::CPPRenamedAttrEnum _n) const;

  inline const ns::capi::AttrOpaque1Renamed* AsFFI() const;
  inline ns::capi::AttrOpaque1Renamed* AsFFI();
  inline static const ns::AttrOpaque1Renamed* FromFFI(const ns::capi::AttrOpaque1Renamed* ptr);
  inline static ns::AttrOpaque1Renamed* FromFFI(ns::capi::AttrOpaque1Renamed* ptr);
  inline static void operator delete(void* ptr);
private:
  AttrOpaque1Renamed() = delete;
  AttrOpaque1Renamed(const ns::AttrOpaque1Renamed&) = delete;
  AttrOpaque1Renamed(ns::AttrOpaque1Renamed&&) noexcept = delete;
  AttrOpaque1Renamed operator=(const ns::AttrOpaque1Renamed&) = delete;
  AttrOpaque1Renamed operator=(ns::AttrOpaque1Renamed&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // AttrOpaque1Renamed_D_HPP
