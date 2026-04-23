#ifndef SOMELIB_ImmutableStructOfOpaque_HPP
#define SOMELIB_ImmutableStructOfOpaque_HPP

#include "ImmutableStructOfOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Opaque.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void ImmutableStructOfOpaque_take_in(const somelib::capi::ImmutableStructOfOpaque* self, somelib::diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string somelib::ImmutableStructOfOpaque::take_in() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    auto thisDiplomatRefClone = this->AsFFI();
    somelib::capi::ImmutableStructOfOpaque_take_in(&thisDiplomatRefClone,
        &write);
    return output;
}
template<typename W>
inline void somelib::ImmutableStructOfOpaque::take_in_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto thisDiplomatRefClone = this->AsFFI();
    somelib::capi::ImmutableStructOfOpaque_take_in(&thisDiplomatRefClone,
        &write);
}


inline somelib::capi::ImmutableStructOfOpaque somelib::ImmutableStructOfOpaque::AsFFI() const {
    return somelib::capi::ImmutableStructOfOpaque {
        /* .i = */ i.AsFFI(),
    };
}

inline somelib::ImmutableStructOfOpaque somelib::ImmutableStructOfOpaque::FromFFI(somelib::capi::ImmutableStructOfOpaque c_struct) {
    return somelib::ImmutableStructOfOpaque {
        /* .i = */ *somelib::Opaque::FromFFI(c_struct.i),
    };
}


#endif // SOMELIB_ImmutableStructOfOpaque_HPP
