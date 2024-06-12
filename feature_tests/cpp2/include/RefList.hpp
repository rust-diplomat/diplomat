#ifndef RefList_HPP
#define RefList_HPP

#include "RefList.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "RefListParameter.hpp"


namespace capi {
    extern "C" {
    
    RefList* RefList_node(const RefListParameter* data);
    
    
    void RefList_destroy(RefList* self);
    
    } // extern "C"
}

inline std::unique_ptr<RefList> RefList::node(const RefListParameter& data) {
  auto result = capi::RefList_node(data.AsFFI());
  return std::unique_ptr<RefList>(RefList::FromFFI(result));
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

inline void RefList::operator delete(void* ptr) {
  capi::RefList_destroy(reinterpret_cast<capi::RefList*>(ptr));
}


#endif // RefList_HPP
