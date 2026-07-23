#ifndef SOMELIB_MutableCallbackHolder_D_HPP
#define SOMELIB_MutableCallbackHolder_D_HPP

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
namespace capi { struct MutableCallbackHolder; }
class MutableCallbackHolder;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MutableCallbackHolder;
    extern "C" {
    void MutableCallbackHolder_destroy(MutableCallbackHolder* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class MutableCallbackHolder;
using MutableCallbackHolderRef = somelib::diplomat::Ref<MutableCallbackHolder, const somelib::capi::MutableCallbackHolder>;
using MutableCallbackHolderRefMut = somelib::diplomat::Ref<MutableCallbackHolder, somelib::capi::MutableCallbackHolder>;

class MutableCallbackHolder : public somelib::diplomat::OpaquePointer<MutableCallbackHolder, somelib::capi::MutableCallbackHolder, somelib::capi::MutableCallbackHolder_destroy> {
public:

  inline static somelib::MutableCallbackHolder new_(std::function<int32_t(int32_t)> func);

  inline int32_t call(int32_t a);

};

} // namespace
#endif // SOMELIB_MutableCallbackHolder_D_HPP
