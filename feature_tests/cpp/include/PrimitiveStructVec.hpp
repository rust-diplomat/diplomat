#ifndef SOMELIB_PrimitiveStructVec_HPP
#define SOMELIB_PrimitiveStructVec_HPP

#include "PrimitiveStructVec.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PrimitiveStruct.hpp"
#include "diplomat_runtime.hpp"
#include "ns/RenamedStructWithAttrs.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::PrimitiveStructVec* PrimitiveStructVec_new(void);

    void PrimitiveStructVec_push(somelib::capi::PrimitiveStructVec* self, somelib::capi::PrimitiveStruct value);

    size_t PrimitiveStructVec_len(const somelib::capi::PrimitiveStructVec* self);

    somelib::capi::DiplomatPrimitiveStructView PrimitiveStructVec_as_slice(const somelib::capi::PrimitiveStructVec* self);

    somelib::capi::DiplomatPrimitiveStructViewMut PrimitiveStructVec_as_slice_mut(somelib::capi::PrimitiveStructVec* self);

    somelib::capi::PrimitiveStruct PrimitiveStructVec_get(const somelib::capi::PrimitiveStructVec* self, size_t idx);

    void PrimitiveStructVec_take_slice_from_other_namespace(somelib::ns::capi::DiplomatRenamedStructWithAttrsView _sl);

    void PrimitiveStructVec_destroy(PrimitiveStructVec* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::PrimitiveStructVec> somelib::PrimitiveStructVec::new_() {
    auto result = somelib::capi::PrimitiveStructVec_new();
    return std::unique_ptr<somelib::PrimitiveStructVec>(somelib::PrimitiveStructVec::FromFFI(result));
}

inline void somelib::PrimitiveStructVec::push(somelib::PrimitiveStruct value) {
    somelib::capi::PrimitiveStructVec_push(this->AsFFI(),
        value.AsFFI());
}

inline size_t somelib::PrimitiveStructVec::len() const {
    auto result = somelib::capi::PrimitiveStructVec_len(this->AsFFI());
    return result;
}

inline somelib::diplomat::span<const somelib::PrimitiveStruct> somelib::PrimitiveStructVec::as_slice() const {
    auto result = somelib::capi::PrimitiveStructVec_as_slice(this->AsFFI());
    return somelib::diplomat::span<const somelib::PrimitiveStruct>(reinterpret_cast<const somelib::PrimitiveStruct*>(result.data), result.len);
}

inline somelib::diplomat::span<somelib::PrimitiveStruct> somelib::PrimitiveStructVec::as_slice_mut() {
    auto result = somelib::capi::PrimitiveStructVec_as_slice_mut(this->AsFFI());
    return somelib::diplomat::span<somelib::PrimitiveStruct>(reinterpret_cast<somelib::PrimitiveStruct*>(result.data), result.len);
}

inline somelib::PrimitiveStruct somelib::PrimitiveStructVec::get(size_t idx) const {
    auto result = somelib::capi::PrimitiveStructVec_get(this->AsFFI(),
        idx);
    return somelib::PrimitiveStruct::FromFFI(result);
}

inline void somelib::PrimitiveStructVec::take_slice_from_other_namespace(somelib::diplomat::span<const somelib::ns::RenamedStructWithAttrs> _sl) {
    somelib::capi::PrimitiveStructVec_take_slice_from_other_namespace({reinterpret_cast<const somelib::ns::capi::RenamedStructWithAttrs*>(_sl.data()), _sl.size()});
}

inline const somelib::capi::PrimitiveStructVec* somelib::PrimitiveStructVec::AsFFI() const {
    return reinterpret_cast<const somelib::capi::PrimitiveStructVec*>(this);
}

inline somelib::capi::PrimitiveStructVec* somelib::PrimitiveStructVec::AsFFI() {
    return reinterpret_cast<somelib::capi::PrimitiveStructVec*>(this);
}

inline const somelib::PrimitiveStructVec* somelib::PrimitiveStructVec::FromFFI(const somelib::capi::PrimitiveStructVec* ptr) {
    return reinterpret_cast<const somelib::PrimitiveStructVec*>(ptr);
}

inline somelib::PrimitiveStructVec* somelib::PrimitiveStructVec::FromFFI(somelib::capi::PrimitiveStructVec* ptr) {
    return reinterpret_cast<somelib::PrimitiveStructVec*>(ptr);
}

inline void somelib::PrimitiveStructVec::operator delete(void* ptr) {
    somelib::capi::PrimitiveStructVec_destroy(reinterpret_cast<somelib::capi::PrimitiveStructVec*>(ptr));
}


#endif // SOMELIB_PrimitiveStructVec_HPP
