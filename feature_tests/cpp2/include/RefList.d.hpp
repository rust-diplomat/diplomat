#ifndef RefList_D_HPP
#define RefList_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct RefListParameter RefListParameter; }
class RefListParameter;


namespace diplomat {
namespace capi {
    typedef struct RefList RefList;
} // namespace capi
} // namespace

class RefList {
public:

  inline static std::unique_ptr<RefList> node(const RefListParameter& data);

  inline const diplomat::capi::RefList* AsFFI() const;
  inline diplomat::capi::RefList* AsFFI();
  inline static const RefList* FromFFI(const diplomat::capi::RefList* ptr);
  inline static RefList* FromFFI(diplomat::capi::RefList* ptr);
  inline static void operator delete(void* ptr);
private:
  RefList() = delete;
  RefList(const RefList&) = delete;
  RefList(RefList&&) noexcept = delete;
  RefList operator=(const RefList&) = delete;
  RefList operator=(RefList&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // RefList_D_HPP
