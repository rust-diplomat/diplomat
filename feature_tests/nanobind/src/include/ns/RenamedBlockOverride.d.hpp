#ifndef SOMELIB_ns_RenamedBlockOverride_D_HPP
#define SOMELIB_ns_RenamedBlockOverride_D_HPP

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
    struct RenamedBlockOverride;
    extern "C" {
    void namespace_BlockOverride_destroy(RenamedBlockOverride* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {

//Pre Test
class RenamedBlockOverride;
using RenamedBlockOverrideRef = somelib::diplomat::Ref<RenamedBlockOverride, const somelib::ns::capi::RenamedBlockOverride>;
using RenamedBlockOverrideRefMut = somelib::diplomat::Ref<RenamedBlockOverride, somelib::ns::capi::RenamedBlockOverride>;

class RenamedBlockOverride : public somelib::diplomat::OpaquePointer<RenamedBlockOverride, somelib::ns::capi::RenamedBlockOverride, somelib::ns::capi::namespace_BlockOverride_destroy> {
public:


private:
public:
    const static bool custom_bool = false;
    static std::string special_function();
};


//Post Test
} // namespace
#endif // SOMELIB_ns_RenamedBlockOverride_D_HPP
