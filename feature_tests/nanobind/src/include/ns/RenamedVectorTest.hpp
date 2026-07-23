#ifndef SOMELIB_ns_RenamedVectorTest_HPP
#define SOMELIB_ns_RenamedVectorTest_HPP

#include "RenamedVectorTest.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedVectorTest* namespace_VectorTest_new(void);

    size_t namespace_VectorTest_len(const somelib::ns::capi::RenamedVectorTest* self);

    typedef struct namespace_VectorTest_get_result {union {double ok; }; bool is_ok;} namespace_VectorTest_get_result;
    namespace_VectorTest_get_result namespace_VectorTest_get(const somelib::ns::capi::RenamedVectorTest* self, size_t idx);

    void namespace_VectorTest_push(somelib::ns::capi::RenamedVectorTest* self, double value);

    void namespace_VectorTest_destroy(RenamedVectorTest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedVectorTest somelib::ns::RenamedVectorTest::new_() {
    auto result = somelib::ns::capi::namespace_VectorTest_new();
    return somelib::ns::RenamedVectorTest::FromFFI(result);
}

inline size_t somelib::ns::RenamedVectorTest::len() const {
    auto result = somelib::ns::capi::namespace_VectorTest_len(this->AsFFI());
    return result;
}

inline somelib::diplomat::Optional<double> somelib::ns::RenamedVectorTest::operator[](size_t idx) const {
    auto result = somelib::ns::capi::namespace_VectorTest_get(this->AsFFI(),
        idx);
    return result.is_ok ? somelib::diplomat::Optional<double>(result.ok) : somelib::diplomat::Optional<double>(std::nullopt);
}

inline void somelib::ns::RenamedVectorTest::push(double value) {
    somelib::ns::capi::namespace_VectorTest_push(this->AsFFI(),
        value);
}


#endif // SOMELIB_ns_RenamedVectorTest_HPP
