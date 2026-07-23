#ifndef SOMELIB_OpaqueThin_D_HPP
#define SOMELIB_OpaqueThin_D_HPP

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
    struct OpaqueThin;
    extern "C" {
    void OpaqueThin_destroy(OpaqueThin* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThin;
using OpaqueThinRef = somelib::diplomat::Ref<OpaqueThin, const somelib::capi::OpaqueThin>;
using OpaqueThinRefMut = somelib::diplomat::Ref<OpaqueThin, somelib::capi::OpaqueThin>;

class OpaqueThin : public somelib::diplomat::OpaquePointer<OpaqueThin, somelib::capi::OpaqueThin, somelib::capi::OpaqueThin_destroy> {
public:

  inline int32_t a() const;

  inline float b() const;

  inline std::string c() const;
  template<typename W>
  inline void c_write(W& writeable_output) const;

};

} // namespace
#endif // SOMELIB_OpaqueThin_D_HPP
