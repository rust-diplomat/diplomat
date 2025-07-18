#ifndef CyclicStructA_HPP
#define CyclicStructA_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::CyclicStructB CyclicStructA_get_b(void);

    void CyclicStructA_cyclic_out(diplomat::capi::CyclicStructA self, diplomat::capi::DiplomatWrite* write);

    uint8_t CyclicStructA_nested_slice(diplomat::capi::DiplomatCyclicStructAView sl);

    void CyclicStructA_double_cyclic_out(diplomat::capi::CyclicStructA self, diplomat::capi::CyclicStructA cyclic_struct_a, diplomat::capi::DiplomatWrite* write);

    void CyclicStructA_getter_out(diplomat::capi::CyclicStructA self, diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline CyclicStructB CyclicStructA::get_b() {
  auto result = diplomat::capi::CyclicStructA_get_b();
  return CyclicStructB::FromFFI(result);
}

inline std::string CyclicStructA::cyclic_out() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::CyclicStructA_cyclic_out(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void CyclicStructA::cyclic_out_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::CyclicStructA_cyclic_out(this->AsFFI(),
    &write);
}

inline uint8_t CyclicStructA::nested_slice(diplomat::span<const CyclicStructA> sl) {
  auto result = diplomat::capi::CyclicStructA_nested_slice({reinterpret_cast<const diplomat::capi::CyclicStructA*>(sl.data()), sl.size()});
  return result;
}

inline std::string CyclicStructA::double_cyclic_out(CyclicStructA cyclic_struct_a) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::CyclicStructA_double_cyclic_out(this->AsFFI(),
    cyclic_struct_a.AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void CyclicStructA::double_cyclic_out_write(CyclicStructA cyclic_struct_a, W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::CyclicStructA_double_cyclic_out(this->AsFFI(),
    cyclic_struct_a.AsFFI(),
    &write);
}

inline std::string CyclicStructA::getter_out() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::CyclicStructA_getter_out(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void CyclicStructA::getter_out_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::CyclicStructA_getter_out(this->AsFFI(),
    &write);
}


inline diplomat::capi::CyclicStructA CyclicStructA::AsFFI() const {
  return diplomat::capi::CyclicStructA {
    /* .a = */ a.AsFFI(),
  };
}

inline CyclicStructA CyclicStructA::FromFFI(diplomat::capi::CyclicStructA c_struct) {
  return CyclicStructA {
    /* .a = */ CyclicStructB::FromFFI(c_struct.a),
  };
}


#endif // CyclicStructA_HPP
