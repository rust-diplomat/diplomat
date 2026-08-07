#ifndef SOMELIB_TraitWrapper_D_HPP
#define SOMELIB_TraitWrapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "TesterTrait.d.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    struct TraitWrapper {
      bool cant_be_empty;
    };

    typedef struct TraitWrapper_option {union { TraitWrapper ok; }; bool is_ok; } TraitWrapper_option;
} // namespace capi
} // namespace


namespace somelib {
struct TraitWrapper {
    bool cant_be_empty;

  inline static int32_t test_with_trait(std::unique_ptr<somelib::TesterTrait> t, int32_t x);

  inline static int32_t test_trait_with_struct(std::unique_ptr<somelib::TesterTrait> t);

  inline static void test_result_output(std::unique_ptr<somelib::TesterTrait> t);

  inline static void test_optional_output(std::unique_ptr<somelib::TesterTrait> t, uint32_t x);

  inline static somelib::diplomat::result<uint32_t, std::optional<uint32_t>> test_result_of_optional(std::unique_ptr<somelib::TesterTrait> t, bool is_ok);

    inline somelib::capi::TraitWrapper AsFFI() const;
    inline static somelib::TraitWrapper FromFFI(somelib::capi::TraitWrapper c_struct);
};

} // namespace
#endif // SOMELIB_TraitWrapper_D_HPP
