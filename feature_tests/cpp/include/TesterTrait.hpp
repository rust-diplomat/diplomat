#ifndef SOMELIB_TesterTrait_HPP
#define SOMELIB_TesterTrait_HPP

#include "TesterTrait.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "TraitTestingStruct.hpp"
#include "diplomat_runtime.hpp"


inline somelib::capi::DiplomatTraitStruct_TesterTrait somelib::TesterTrait::AsFFI() const {
    struct somelib::capi::DiplomatTraitStruct_TesterTrait trait_inner = {
        (void*)this,
        {
            &somelib::TesterTrait::Destroy,
            somelib::capi::TesterTrait_DATA_SIZE,
            somelib::capi::TesterTrait_DATA_ALIGNMENT,
            [](void* self, uint32_t x) -> diplomat::replace_fn_t<uint32_t> {
                return somelib::diplomat::fn_trait_helpers::replace_ret(reinterpret_cast<somelib::TesterTrait*>(self)->test_trait_fn(somelib::diplomat::fn_trait_helpers::replace<uint32_t>(x)));
            },
            [](void* self) -> diplomat::replace_fn_t<void> {
                return reinterpret_cast<somelib::TesterTrait*>(self)->test_void_trait_fn();
            },
            [](void* self, somelib::capi::TraitTestingStruct s) -> diplomat::replace_fn_t<int32_t> {
                return somelib::diplomat::fn_trait_helpers::replace_ret(reinterpret_cast<somelib::TesterTrait*>(self)->test_struct_trait_fn(somelib::diplomat::fn_trait_helpers::replace<somelib::TraitTestingStruct>(s)));
            },
            [](void* self) -> somelib::capi::test_result_output_result {
                return somelib::diplomat::fn_trait_helpers::replace_result<uint32_t, std::monostate, somelib::capi::test_result_output_result>(reinterpret_cast<somelib::TesterTrait*>(self)->test_result_output());
            },
            [](void* self, uint32_t x) -> somelib::capi::test_optional_output_result {
                return somelib::diplomat::fn_trait_helpers::replace_optional_ret<somelib::capi::test_optional_output_result, uint32_t>(reinterpret_cast<somelib::TesterTrait*>(self)->test_optional_output(somelib::diplomat::fn_trait_helpers::replace<uint32_t>(x)));
            },
            [](void* self, bool is_ok) -> somelib::capi::test_result_of_optional_result {
                return somelib::diplomat::fn_trait_helpers::replace_result<uint32_t, std::optional<uint32_t>, somelib::capi::test_result_of_optional_result>(reinterpret_cast<somelib::TesterTrait*>(self)->test_result_of_optional(somelib::diplomat::fn_trait_helpers::replace<bool>(is_ok)));
            },
        }
    };
    return trait_inner;
}

void somelib::TesterTrait::Destroy(const void* data) {
    auto self = static_cast<const somelib::TesterTrait*>(data);
    delete self;
}
#endif // SOMELIB_TesterTrait_HPP
