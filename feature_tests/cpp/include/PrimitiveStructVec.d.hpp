#ifndef SOMELIB_PrimitiveStructVec_D_HPP
#define SOMELIB_PrimitiveStructVec_D_HPP

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
namespace capi { struct PrimitiveStructVec; }
class PrimitiveStructVec;
struct PrimitiveStruct;
} // namespace somelib
namespace somelib {
namespace ns {
struct RenamedStructWithAttrs;
} // namespace ns
} // namespace somelib



namespace somelib {
namespace capi {
    struct PrimitiveStructVec;
    extern "C" {
    void PrimitiveStructVec_destroy(PrimitiveStructVec* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class PrimitiveStructVec;
using PrimitiveStructVecRef = somelib::diplomat::Ref<PrimitiveStructVec, const somelib::capi::PrimitiveStructVec>;
using PrimitiveStructVecRefMut = somelib::diplomat::Ref<PrimitiveStructVec, somelib::capi::PrimitiveStructVec>;

class PrimitiveStructVec : public somelib::diplomat::OpaquePointer<PrimitiveStructVec, somelib::capi::PrimitiveStructVec, somelib::capi::PrimitiveStructVec_destroy> {
public:

  inline static somelib::PrimitiveStructVec new_();

  inline void push(somelib::PrimitiveStruct value);

  inline size_t len() const;

  inline somelib::diplomat::span<const somelib::PrimitiveStruct> as_slice() const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::span<somelib::PrimitiveStruct> as_slice_mut() DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::Optional<somelib::PrimitiveStruct> get(size_t idx) const;

  inline static void take_slice_from_other_namespace(somelib::diplomat::span<const somelib::ns::RenamedStructWithAttrs> _sl);

  inline static somelib::PrimitiveStructVec take_in_slice(somelib::diplomat::span<const somelib::PrimitiveStruct> a);

};

} // namespace
#endif // SOMELIB_PrimitiveStructVec_D_HPP
