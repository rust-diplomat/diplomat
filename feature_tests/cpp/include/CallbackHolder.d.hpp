#ifndef SOMELIB_CallbackHolder_D_HPP
#define SOMELIB_CallbackHolder_D_HPP

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
namespace capi { struct CallbackHolder; }
class CallbackHolder;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CallbackHolder;
    extern "C" {
    void CallbackHolder_destroy(CallbackHolder* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class CallbackHolder;
using CallbackHolderRef = somelib::diplomat::Ref<CallbackHolder, const somelib::capi::CallbackHolder>;
using CallbackHolderRefMut = somelib::diplomat::Ref<CallbackHolder, somelib::capi::CallbackHolder>;

class CallbackHolder : public somelib::diplomat::OpaquePointer<CallbackHolder, somelib::capi::CallbackHolder, somelib::capi::CallbackHolder_destroy> {
public:

  inline static somelib::CallbackHolder new_(std::function<int32_t(int32_t)> func);

  inline int32_t call(int32_t a) const;

};

} // namespace
#endif // SOMELIB_CallbackHolder_D_HPP
