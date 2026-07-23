#ifndef SOMELIB_RefList_D_HPP
#define SOMELIB_RefList_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct RefList; }
class RefList;
namespace capi { struct RefListParameter; }
class RefListParameter;
} // namespace somelib



namespace somelib {
namespace capi {
    struct RefList;
    extern "C" {
    void RefList_destroy(RefList* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class RefList;
using RefListRef = somelib::diplomat::Ref<RefList, const somelib::capi::RefList>;
using RefListRefMut = somelib::diplomat::Ref<RefList, somelib::capi::RefList>;

class RefList : public somelib::diplomat::OpaquePointer<RefList, somelib::capi::RefList, somelib::capi::RefList_destroy> {
public:

  inline static somelib::RefList node(const somelib::RefListParameter& data DIPLOMAT_LIFETIME_BOUND);

};

} // namespace
#endif // SOMELIB_RefList_D_HPP
