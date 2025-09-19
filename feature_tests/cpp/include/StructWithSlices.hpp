#ifndef SOMELIB_StructWithSlices_HPP
#define SOMELIB_StructWithSlices_HPP

#include "StructWithSlices.d.hpp"

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
namespace capi {
    extern "C" {

    void StructWithSlices_return_last(somelib::capi::StructWithSlices self, somelib::diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string somelib::StructWithSlices::return_last() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::StructWithSlices_return_last(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::StructWithSlices::return_last_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::StructWithSlices_return_last(this->AsFFI(),
        &write);
}


inline somelib::capi::StructWithSlices somelib::StructWithSlices::AsFFI() const {
    return somelib::capi::StructWithSlices {
        /* .first = */ {first.data(), first.size()},
        /* .second = */ {second.data(), second.size()},
    };
}

inline somelib::StructWithSlices somelib::StructWithSlices::FromFFI(somelib::capi::StructWithSlices c_struct) {
    return somelib::StructWithSlices {
        /* .first = */ std::string_view(c_struct.first.data, c_struct.first.len),
        /* .second = */ somelib::diplomat::span<const uint16_t>(c_struct.second.data, c_struct.second.len),
    };
}


#endif // SOMELIB_StructWithSlices_HPP
