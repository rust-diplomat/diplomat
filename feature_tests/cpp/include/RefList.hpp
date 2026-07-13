#ifndef SOMELIB_RefList_HPP
#define SOMELIB_RefList_HPP

#include "RefList.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "RefListParameter.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::RefList* RefList_node(const somelib::capi::RefListParameter* data);

    void RefList_destroy(RefList* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::RefList somelib::RefList::node(const somelib::RefListParameter& data DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::RefList_node(data.AsFFI());
    return somelib::RefList::FromFFI(result);
}


#endif // SOMELIB_RefList_HPP
