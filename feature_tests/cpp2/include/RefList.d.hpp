#ifndef RefList_D_HPP
#define RefList_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "RefList.d.h"

class RefListParameter;


class RefList {
public:

  inline static std::unique_ptr<RefList> node(const RefListParameter& data);

  inline const capi::RefList* AsFFI() const;
  inline capi::RefList* AsFFI();
  inline static const RefList* FromFFI(const capi::RefList* ptr);
  inline static RefList* FromFFI(capi::RefList* ptr);
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
