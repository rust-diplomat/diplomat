#ifndef ns_RenamedMyIndexer_HPP
#define ns_RenamedMyIndexer_HPP

#include "RenamedMyIndexer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    
    typedef struct namespace_MyIndexer_get_result {union {diplomat::capi::DiplomatStringView ok; }; bool is_ok;} namespace_MyIndexer_get_result;
    namespace_MyIndexer_get_result namespace_MyIndexer_get(const ns::capi::RenamedMyIndexer* self, size_t i);
    
    
    void namespace_MyIndexer_destroy(RenamedMyIndexer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<std::string_view> ns::RenamedMyIndexer::operator[](size_t i) const {
  auto result = ns::capi::namespace_MyIndexer_get(this->AsFFI(),
    i);
  return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline const ns::capi::RenamedMyIndexer* ns::RenamedMyIndexer::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedMyIndexer*>(this);
}

inline ns::capi::RenamedMyIndexer* ns::RenamedMyIndexer::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedMyIndexer*>(this);
}

inline const ns::RenamedMyIndexer* ns::RenamedMyIndexer::FromFFI(const ns::capi::RenamedMyIndexer* ptr) {
  return reinterpret_cast<const ns::RenamedMyIndexer*>(ptr);
}

inline ns::RenamedMyIndexer* ns::RenamedMyIndexer::FromFFI(ns::capi::RenamedMyIndexer* ptr) {
  return reinterpret_cast<ns::RenamedMyIndexer*>(ptr);
}

inline void ns::RenamedMyIndexer::operator delete(void* ptr) {
  ns::capi::namespace_MyIndexer_destroy(reinterpret_cast<ns::capi::RenamedMyIndexer*>(ptr));
}


#endif // ns_RenamedMyIndexer_HPP
