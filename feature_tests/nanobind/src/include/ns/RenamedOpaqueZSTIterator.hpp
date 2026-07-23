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

inline somelib::ns::RenamedOpaqueZSTIterator somelib::ns::RenamedOpaqueZSTIterator::ctor() {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_ctor();
    return somelib::ns::RenamedOpaqueZSTIterator::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZSTIterator::next() const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_next(this->AsFFI());
    return somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIterator>::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZSTIterator::operator[](size_t _idx) const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_nullable_indexer(this->AsFFI(),
        _idx);
    return somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIterator>::FromFFI(result);
}

inline somelib::diplomat::result<std::string, somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZSTIterator::stringify() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_stringify(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::string, somelib::ns::RenamedOpaqueZST>(somelib::diplomat::Ok<std::string>(std::move(output))) : somelib::diplomat::result<std::string, somelib::ns::RenamedOpaqueZST>(somelib::diplomat::Err<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err)));
}
template<typename W>
inline somelib::diplomat::result<std::monostate, somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZSTIterator::stringify_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::ns::capi::namespace_OpaqueZSTIterator_stringify(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::ns::RenamedOpaqueZST>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::ns::RenamedOpaqueZST>(somelib::diplomat::Err<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err)));
}


#endif // SOMELIB_ns_RenamedOpaqueZSTIterator_HPP
