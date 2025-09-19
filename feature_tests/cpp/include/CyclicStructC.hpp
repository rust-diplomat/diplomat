#ifndef SOMELIB_CyclicStructC_HPP
#define SOMELIB_CyclicStructC_HPP

#include "CyclicStructC.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CyclicStructA.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::CyclicStructC CyclicStructC_takes_nested_parameters(somelib::capi::CyclicStructC c);

    void CyclicStructC_cyclic_out(somelib::capi::CyclicStructC self, somelib::diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::CyclicStructC somelib::CyclicStructC::takes_nested_parameters(somelib::CyclicStructC c) {
    auto result = somelib::capi::CyclicStructC_takes_nested_parameters(c.AsFFI());
    return somelib::CyclicStructC::FromFFI(result);
}

inline std::string somelib::CyclicStructC::cyclic_out() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CyclicStructC_cyclic_out(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::CyclicStructC::cyclic_out_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CyclicStructC_cyclic_out(this->AsFFI(),
        &write);
}


inline somelib::capi::CyclicStructC somelib::CyclicStructC::AsFFI() const {
    return somelib::capi::CyclicStructC {
        /* .a = */ a.AsFFI(),
    };
}

inline somelib::CyclicStructC somelib::CyclicStructC::FromFFI(somelib::capi::CyclicStructC c_struct) {
    return somelib::CyclicStructC {
        /* .a = */ somelib::CyclicStructA::FromFFI(c_struct.a),
    };
}


#endif // SOMELIB_CyclicStructC_HPP
