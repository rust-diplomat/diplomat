#ifndef SOMELIB_OpaqueThinVec_HPP
#define SOMELIB_OpaqueThinVec_HPP

#include "OpaqueThinVec.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OpaqueThin.hpp"
#include "OpaqueThinIter.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::OpaqueThinVec* OpaqueThinVec_create(somelib::diplomat::capi::DiplomatI32View a, somelib::diplomat::capi::DiplomatF32View b, somelib::diplomat::capi::DiplomatStringView c);

    somelib::capi::OpaqueThinIter* OpaqueThinVec_iter(const somelib::capi::OpaqueThinVec* self);

    size_t OpaqueThinVec_len(const somelib::capi::OpaqueThinVec* self);

    const somelib::capi::OpaqueThin* OpaqueThinVec_get(const somelib::capi::OpaqueThinVec* self, size_t idx);

    const somelib::capi::OpaqueThin* OpaqueThinVec_first(const somelib::capi::OpaqueThinVec* self);

    void OpaqueThinVec_destroy(OpaqueThinVec* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::OpaqueThinVec> somelib::OpaqueThinVec::create(somelib::diplomat::span<const int32_t> a, somelib::diplomat::span<const float> b, std::string_view c) {
    auto result = somelib::capi::OpaqueThinVec_create({a.data(), a.size()},
        {b.data(), b.size()},
        {c.data(), c.size()});
    return std::unique_ptr<somelib::OpaqueThinVec>(somelib::OpaqueThinVec::FromFFI(result));
}

inline std::unique_ptr<somelib::OpaqueThinIter> somelib::OpaqueThinVec::iter() const {
    auto result = somelib::capi::OpaqueThinVec_iter(this->AsFFI());
    return std::unique_ptr<somelib::OpaqueThinIter>(somelib::OpaqueThinIter::FromFFI(result));
}

inline somelib::diplomat::next_to_iter_helper<somelib::OpaqueThinIter>somelib::OpaqueThinVec::begin() const {
    return iter();
}

inline size_t somelib::OpaqueThinVec::__len__() const {
    auto result = somelib::capi::OpaqueThinVec_len(this->AsFFI());
    return result;
}

inline const somelib::OpaqueThin* somelib::OpaqueThinVec::operator[](size_t idx) const {
    auto result = somelib::capi::OpaqueThinVec_get(this->AsFFI(),
        idx);
    return somelib::OpaqueThin::FromFFI(result);
}

inline const somelib::OpaqueThin* somelib::OpaqueThinVec::first() const {
    auto result = somelib::capi::OpaqueThinVec_first(this->AsFFI());
    return somelib::OpaqueThin::FromFFI(result);
}

inline const somelib::capi::OpaqueThinVec* somelib::OpaqueThinVec::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueThinVec*>(this);
}

inline somelib::capi::OpaqueThinVec* somelib::OpaqueThinVec::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueThinVec*>(this);
}

inline const somelib::OpaqueThinVec* somelib::OpaqueThinVec::FromFFI(const somelib::capi::OpaqueThinVec* ptr) {
    return reinterpret_cast<const somelib::OpaqueThinVec*>(ptr);
}

inline somelib::OpaqueThinVec* somelib::OpaqueThinVec::FromFFI(somelib::capi::OpaqueThinVec* ptr) {
    return reinterpret_cast<somelib::OpaqueThinVec*>(ptr);
}

inline void somelib::OpaqueThinVec::operator delete(void* ptr) {
    somelib::capi::OpaqueThinVec_destroy(reinterpret_cast<somelib::capi::OpaqueThinVec*>(ptr));
}


#endif // SOMELIB_OpaqueThinVec_HPP
