#ifndef RefList_HPP
#define RefList_HPP

#include "RefList.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "RefListParameter.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::RefList* RefList_node(const diplomat::capi::RefListParameter* data);
    
    
    void RefList_destroy(RefList* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<RefList> RefList::node(const RefListParameter& data) {
  auto result = diplomat::capi::RefList_node(data.AsFFI());
  return std::unique_ptr<RefList>(RefList::FromFFI(result));
}

inline const diplomat::capi::RefList* RefList::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::RefList*>(this);
}

inline diplomat::capi::RefList* RefList::AsFFI() {
  return reinterpret_cast<diplomat::capi::RefList*>(this);
}

inline const RefList* RefList::FromFFI(const diplomat::capi::RefList* ptr) {
  return reinterpret_cast<const RefList*>(ptr);
}

inline RefList* RefList::FromFFI(diplomat::capi::RefList* ptr) {
  return reinterpret_cast<RefList*>(ptr);
}

inline void RefList::operator delete(void* ptr) {
  diplomat::capi::RefList_destroy(reinterpret_cast<diplomat::capi::RefList*>(ptr));
}


#endif // RefList_HPP
