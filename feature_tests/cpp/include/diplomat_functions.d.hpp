#ifndef diplomat_functions_D_HPP
#define diplomat_functions_D_HPP

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
    typedef struct DiplomatCallback_free_callback_holder_f_result { bool is_ok;} DiplomatCallback_free_callback_holder_f_result;

    typedef struct DiplomatCallback_free_callback_holder_f {
        const void* data;
        DiplomatCallback_free_callback_holder_f_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_free_callback_holder_f;

    void free_callback_holder(DiplomatCallback_free_callback_holder_f f_cb_wrap);

    } // extern "C"
} // namespace capi
} // namespace




    
inline static void free_callback_holder(std::function<diplomat::result<std::monostate, std::monostate>()> f);




#endif // diplomat_functions_D_HPP
