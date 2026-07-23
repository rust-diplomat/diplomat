#ifndef SOMELIB_ns_RenamedStringList_D_HPP
#define SOMELIB_ns_RenamedStringList_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedStringList;
    extern "C" {
    void namespace_StringList_destroy(RenamedStringList* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedStringList;
using RenamedStringListRef = somelib::diplomat::Ref<RenamedStringList, const somelib::ns::capi::RenamedStringList>;
using RenamedStringListRefMut = somelib::diplomat::Ref<RenamedStringList, somelib::ns::capi::RenamedStringList>;

/**
 * Testing support for List[str] in Nanobind
 */
class RenamedStringList : public somelib::diplomat::OpaquePointer<RenamedStringList, somelib::ns::capi::RenamedStringList, somelib::ns::capi::namespace_StringList_destroy> {
public:


private:
public:
    static std::vector<std::string> return_new();
};

} // namespace
#endif // SOMELIB_ns_RenamedStringList_D_HPP
