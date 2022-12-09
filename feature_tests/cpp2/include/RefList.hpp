#ifndef RefList_HPP
#define RefList_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "RefList.d.hpp"
#include "RefList.h"





inline std::unique_ptr<RefList> RefList::node(const RefListParameter& data) {
  // TODO
}

inline const capi::RefList* RefList::AsFFI() const {
  return reinterpret_cast<const capi::RefList*>(this);
}
inline capi::RefList* RefList::AsFFI() {
  return reinterpret_cast<capi::RefList*>(this);
}
inline RefList::~RefList() {
  capi::RefList_destroy(AsFFI());
}


#endif // RefList_HPP
