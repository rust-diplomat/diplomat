#ifndef SOMELIB_CyclicStructA_HPP
#define SOMELIB_CyclicStructA_HPP

#include "CyclicStructA.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CyclicStructB.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::CyclicStructB CyclicStructA_get_b(void);

    void CyclicStructA_cyclic_out(somelib::capi::CyclicStructA self, somelib::diplomat::capi::DiplomatWrite* write);

    uint8_t CyclicStructA_nested_slice(somelib::capi::DiplomatCyclicStructAView sl);

    void CyclicStructA_double_cyclic_out(somelib::capi::CyclicStructA self, somelib::capi::CyclicStructA cyclic_struct_a, somelib::diplomat::capi::DiplomatWrite* write);

    void CyclicStructA_getter_out(somelib::capi::CyclicStructA self, somelib::diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::CyclicStructB somelib::CyclicStructA::get_b() {
    auto result = somelib::capi::CyclicStructA_get_b();
    return somelib::CyclicStructB::FromFFI(result);
}

inline std::string somelib::CyclicStructA::cyclic_out() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CyclicStructA_cyclic_out(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::CyclicStructA::cyclic_out_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CyclicStructA_cyclic_out(this->AsFFI(),
        &write);
}

inline uint8_t somelib::CyclicStructA::nested_slice(somelib::diplomat::span<const somelib::CyclicStructA> sl) {
    auto result = somelib::capi::CyclicStructA_nested_slice({reinterpret_cast<const somelib::capi::CyclicStructA*>(sl.data()), sl.size()});
    return result;
}

inline std::string somelib::CyclicStructA::double_cyclic_out(somelib::CyclicStructA cyclic_struct_a) const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CyclicStructA_double_cyclic_out(this->AsFFI(),
        cyclic_struct_a.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::CyclicStructA::double_cyclic_out_write(somelib::CyclicStructA cyclic_struct_a, W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CyclicStructA_double_cyclic_out(this->AsFFI(),
        cyclic_struct_a.AsFFI(),
        &write);
}

inline std::string somelib::CyclicStructA::getter_out() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CyclicStructA_getter_out(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::CyclicStructA::getter_out_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CyclicStructA_getter_out(this->AsFFI(),
        &write);
}


inline somelib::capi::CyclicStructA somelib::CyclicStructA::AsFFI() const {
    return somelib::capi::CyclicStructA {
        /* .a = */ a.AsFFI(),
    };
}

inline somelib::CyclicStructA somelib::CyclicStructA::FromFFI(somelib::capi::CyclicStructA c_struct) {
    return somelib::CyclicStructA {
        /* .a = */ somelib::CyclicStructB::FromFFI(c_struct.a),
    };
}


#endif // SOMELIB_CyclicStructA_HPP
