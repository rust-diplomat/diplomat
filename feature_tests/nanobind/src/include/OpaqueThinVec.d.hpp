#ifndef SOMELIB_OpaqueThinVec_D_HPP
#define SOMELIB_OpaqueThinVec_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OpaqueThin.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct OpaqueThin; }
class OpaqueThin;
namespace capi { struct OpaqueThinIter; }
class OpaqueThinIter;
namespace capi { struct OpaqueThinVec; }
class OpaqueThinVec;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueThinVec;
    extern "C" {
    void OpaqueThinVec_destroy(OpaqueThinVec* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThinVec;
using OpaqueThinVecRef = somelib::diplomat::Ref<OpaqueThinVec, const somelib::capi::OpaqueThinVec>;
using OpaqueThinVecRefMut = somelib::diplomat::Ref<OpaqueThinVec, somelib::capi::OpaqueThinVec>;

class OpaqueThinVec : public somelib::diplomat::OpaquePointer<OpaqueThinVec, somelib::capi::OpaqueThinVec, somelib::capi::OpaqueThinVec_destroy> {
public:

  inline static somelib::OpaqueThinVec create(somelib::diplomat::span<const int32_t> a, somelib::diplomat::span<const float> b, std::string_view c);

  inline somelib::OpaqueThinIter iter() const DIPLOMAT_LIFETIME_BOUND;
  inline somelib::diplomat::next_to_iter_helper<somelib::OpaqueThinIter> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline size_t __len__() const;

  inline somelib::diplomat::Optional<somelib::OpaqueThinRef> operator[](size_t idx) const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::Optional<somelib::OpaqueThinRef> first() const DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_OpaqueThinVec_D_HPP
