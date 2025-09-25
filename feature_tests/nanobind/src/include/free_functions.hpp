#ifndef free_functions_HPP
#define free_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_diplomat_external_free_callback_holder_f_result { bool is_ok;} DiplomatCallback_diplomat_external_free_callback_holder_f_result;

    typedef struct DiplomatCallback_diplomat_external_free_callback_holder_f {
        const void* data;
        DiplomatCallback_diplomat_external_free_callback_holder_f_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_diplomat_external_free_callback_holder_f;

    void diplomat_external_free_callback_holder(DiplomatCallback_diplomat_external_free_callback_holder_f f_cb_wrap);

    } // extern "C"
} // namespace capi
} // namespace


inline void free_callback_holder(std::function<diplomat::result<std::monostate, std::monostate>()> f) {
    diplomat::capi::diplomat_external_free_callback_holder({new decltype(f)(std::move(f)), diplomat::fn_traits(f).template c_run_callback_result<std::monostate, std::monostate, diplomat::capi::DiplomatCallback_diplomat_external_free_callback_holder_f_result>, diplomat::fn_traits(f).c_delete});
}


#endif // free_functions_HPP
