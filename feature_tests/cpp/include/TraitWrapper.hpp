#ifndef SOMELIB_TraitWrapper_HPP
#define SOMELIB_TraitWrapper_HPP

#include "TraitWrapper.d.hpp"

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
    extern "C" {

    int32_t TraitWrapper_test_with_trait(somelib::capi::DiplomatTraitStruct_TesterTrait t_trait_wrap, int32_t x);

    int32_t TraitWrapper_test_trait_with_struct(somelib::capi::DiplomatTraitStruct_TesterTrait t_trait_wrap);

    void TraitWrapper_test_result_output(somelib::capi::DiplomatTraitStruct_TesterTrait t_trait_wrap);

    void TraitWrapper_test_optional_output(somelib::capi::DiplomatTraitStruct_TesterTrait t_trait_wrap, uint32_t x);

    typedef struct TraitWrapper_test_result_of_optional_result {union {uint32_t ok; somelib::diplomat::capi::OptionU32 err;}; bool is_ok;} TraitWrapper_test_result_of_optional_result;
    TraitWrapper_test_result_of_optional_result TraitWrapper_test_result_of_optional(somelib::capi::DiplomatTraitStruct_TesterTrait t_trait_wrap, bool is_ok);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t somelib::TraitWrapper::test_with_trait(std::unique_ptr<somelib::TesterTrait> t, int32_t x) {
    auto result = somelib::capi::TraitWrapper_test_with_trait(t.release()->AsFFI(),
        x);
    return result;
}

inline int32_t somelib::TraitWrapper::test_trait_with_struct(std::unique_ptr<somelib::TesterTrait> t) {
    auto result = somelib::capi::TraitWrapper_test_trait_with_struct(t.release()->AsFFI());
    return result;
}

inline void somelib::TraitWrapper::test_result_output(std::unique_ptr<somelib::TesterTrait> t) {
    somelib::capi::TraitWrapper_test_result_output(t.release()->AsFFI());
}

inline void somelib::TraitWrapper::test_optional_output(std::unique_ptr<somelib::TesterTrait> t, uint32_t x) {
    somelib::capi::TraitWrapper_test_optional_output(t.release()->AsFFI(),
        x);
}

inline somelib::diplomat::result<uint32_t, std::optional<uint32_t>> somelib::TraitWrapper::test_result_of_optional(std::unique_ptr<somelib::TesterTrait> t, bool is_ok) {
    auto result = somelib::capi::TraitWrapper_test_result_of_optional(t.release()->AsFFI(),
        is_ok);
    return result.is_ok ? somelib::diplomat::result<uint32_t, std::optional<uint32_t>>(somelib::diplomat::Ok<uint32_t>(result.ok)) : somelib::diplomat::result<uint32_t, std::optional<uint32_t>>(somelib::diplomat::Err<std::optional<uint32_t>>(result.err.is_ok ? std::optional(result.err.ok) : std::nullopt));
}


inline somelib::capi::TraitWrapper somelib::TraitWrapper::AsFFI() const {
    return somelib::capi::TraitWrapper {
        /* .cant_be_empty = */ cant_be_empty,
    };
}

inline somelib::TraitWrapper somelib::TraitWrapper::FromFFI(somelib::capi::TraitWrapper c_struct) {
    return somelib::TraitWrapper {
        /* .cant_be_empty = */ c_struct.cant_be_empty,
    };
}


#endif // SOMELIB_TraitWrapper_HPP
