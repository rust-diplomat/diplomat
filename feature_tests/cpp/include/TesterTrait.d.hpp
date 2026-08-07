#ifndef SOMELIB_TesterTrait_D_HPP
#define SOMELIB_TesterTrait_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "TraitTestingStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct TraitTestingStruct;
} // namespace somelib



namespace somelib {
namespace capi {
    typedef struct test_result_output_result {union {uint32_t ok; }; bool is_ok;} test_result_output_result;

    typedef struct test_optional_output_result {union {uint32_t ok; }; bool is_ok;} test_optional_output_result;

    typedef struct test_result_of_optional_result {union {uint32_t ok; somelib::diplomat::capi::OptionU32 err;}; bool is_ok;} test_result_of_optional_result;
    struct TesterTrait_VTable {
        void (*destructor)(const void*);
        size_t SIZE; size_t ALIGNMENT;
        uint32_t (*run_test_trait_fn_callback)(void*, uint32_t);
        void (*run_test_void_trait_fn_callback)(void*);
        int32_t (*run_test_struct_trait_fn_callback)(void*, somelib::capi::TraitTestingStruct);
        test_result_output_result (*run_test_result_output_callback)(void*);
        test_optional_output_result (*run_test_optional_output_callback)(void*, uint32_t);
        test_result_of_optional_result (*run_test_result_of_optional_callback)(void*, bool);
    };

    struct DiplomatTraitStruct_TesterTrait {
        void* data;
        TesterTrait_VTable vtable;
    };

    static void general_destructor(const void* data) {
        // TODO
    }

    const size_t TesterTrait_DATA_SIZE = 0;
    const size_t TesterTrait_DATA_ALIGNMENT = 0;
} // namespace capi
} // namespace

namespace somelib {
class TesterTrait {
    private:
    static void Destroy(const void* data);

    protected:
    virtual uint32_t test_trait_fn(uint32_t x) = 0;
    virtual void test_void_trait_fn() = 0;
    virtual int32_t test_struct_trait_fn(somelib::TraitTestingStruct s) = 0;
    virtual somelib::diplomat::result<uint32_t, std::monostate> test_result_output() = 0;
    virtual std::optional<uint32_t> test_optional_output(uint32_t x) = 0;
    virtual somelib::diplomat::result<uint32_t, std::optional<uint32_t>> test_result_of_optional(bool is_ok) = 0;

    public:
    inline somelib::capi::DiplomatTraitStruct_TesterTrait AsFFI() const;
};


} // namespace
#endif // SOMELIB_TesterTrait_D_HPP
