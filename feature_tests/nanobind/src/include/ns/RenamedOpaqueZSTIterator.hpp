#ifndef SOMELIB_ns_RenamedOpaqueZSTIterator_HPP
#define SOMELIB_ns_RenamedOpaqueZSTIterator_HPP

#include "RenamedOpaqueZSTIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueZST.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedOpaqueZSTIterator* namespace_OpaqueZSTIterator_ctor(void);

    somelib::ns::capi::RenamedOpaqueZSTIterator* namespace_OpaqueZSTIterator_next(const somelib::ns::capi::RenamedOpaqueZSTIterator* self);

    somelib::ns::capi::RenamedOpaqueZSTIterator* namespace_OpaqueZSTIterator_nullable_indexer(const somelib::ns::capi::RenamedOpaqueZSTIterator* self, size_t _idx);

    typedef struct namespace_OpaqueZSTIterator_stringify_result {union { somelib::ns::capi::RenamedOpaqueZST* err;}; bool is_ok;} namespace_OpaqueZSTIterator_stringify_result;
    namespace_OpaqueZSTIterator_stringify_result namespace_OpaqueZSTIterator_stringify(const somelib::ns::capi::RenamedOpaqueZSTIterator* self, somelib::diplomat::capi::DiplomatWrite* write);

    void namespace_OpaqueZSTIterator_destroy(RenamedOpaqueZSTIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZSTIterator::ctor() {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_ctor();
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator>(somelib::ns::RenamedOpaqueZSTIterator::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZSTIterator::next() const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_next(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator>(somelib::ns::RenamedOpaqueZSTIterator::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZSTIterator::operator[](size_t _idx) const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_nullable_indexer(this->AsFFI(),
        _idx);
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator>(somelib::ns::RenamedOpaqueZSTIterator::FromFFI(result));
}

inline somelib::diplomat::result<std::string, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> somelib::ns::RenamedOpaqueZSTIterator::stringify() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_stringify(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::string, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Ok<std::string>(std::move(output))) : somelib::diplomat::result<std::string, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Err<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err))));
}
template<typename W>
inline somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> somelib::ns::RenamedOpaqueZSTIterator::stringify_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_stringify(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Err<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err))));
}

inline const somelib::ns::capi::RenamedOpaqueZSTIterator* somelib::ns::RenamedOpaqueZSTIterator::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueZSTIterator*>(this);
}

inline somelib::ns::capi::RenamedOpaqueZSTIterator* somelib::ns::RenamedOpaqueZSTIterator::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueZSTIterator*>(this);
}

inline const somelib::ns::RenamedOpaqueZSTIterator* somelib::ns::RenamedOpaqueZSTIterator::FromFFI(const somelib::ns::capi::RenamedOpaqueZSTIterator* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueZSTIterator*>(ptr);
}

inline somelib::ns::RenamedOpaqueZSTIterator* somelib::ns::RenamedOpaqueZSTIterator::FromFFI(somelib::ns::capi::RenamedOpaqueZSTIterator* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueZSTIterator*>(ptr);
}

inline void somelib::ns::RenamedOpaqueZSTIterator::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueZSTIterator_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueZSTIterator*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueZSTIterator_HPP
