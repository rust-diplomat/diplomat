#ifndef RefList_HPP
#define RefList_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "RefList.d.hpp"
#include "RefList.h"
#include "RefListParameter.d.hpp"





inline std::unique_ptr<RefList> RefList::node(const RefListParameter& data) {
  auto result = capi::RefList_node(data.AsFFI());
  return std::unique_ptr(RefList::FromFFI(result));
}

inline const capi::RefList* RefList::AsFFI() const {
  return reinterpret_cast<const capi::RefList*>(this);
}
inline capi::RefList* RefList::AsFFI() {
  return reinterpret_cast<capi::RefList*>(this);
}
inline const RefList* RefList::FromFFI(const capi::RefList* ptr) {
  return reinterpret_cast<const RefList*>(ptr);
}
inline RefList* RefList::FromFFI(capi::RefList* ptr) {
  return reinterpret_cast<RefList*>(ptr);
}
inline RefList::~RefList() {
  capi::RefList_destroy(AsFFI());
}


#endif // RefList_HPP
