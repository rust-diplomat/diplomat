#ifndef SOMELIB_ns_RenamedOpaqueZST_HPP
#define SOMELIB_ns_RenamedOpaqueZST_HPP

#include "RenamedOpaqueZST.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueZSTIterator.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_ctor(void);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_make(void);

    void namespace_OpaqueZST_out_string(somelib::diplomat::capi::DiplomatWrite* write);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_member(const somelib::ns::capi::RenamedOpaqueZST* self);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_mut_member(somelib::ns::capi::RenamedOpaqueZST* self);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_add(const somelib::ns::capi::RenamedOpaqueZST* self, const somelib::ns::capi::RenamedOpaqueZST* _o);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_sub(const somelib::ns::capi::RenamedOpaqueZST* self, const somelib::ns::capi::RenamedOpaqueZST* _o);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_mul(const somelib::ns::capi::RenamedOpaqueZST* self, const somelib::ns::capi::RenamedOpaqueZST* _o);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_div(const somelib::ns::capi::RenamedOpaqueZST* self, const somelib::ns::capi::RenamedOpaqueZST* _o);

    typedef struct namespace_OpaqueZST_success_zst_result {union {somelib::ns::capi::RenamedOpaqueZST* ok; }; bool is_ok;} namespace_OpaqueZST_success_zst_result;
    namespace_OpaqueZST_success_zst_result namespace_OpaqueZST_success_zst(bool return_success);

    typedef struct namespace_OpaqueZST_fail_zst_result {union { somelib::ns::capi::RenamedOpaqueZST* err;}; bool is_ok;} namespace_OpaqueZST_fail_zst_result;
    namespace_OpaqueZST_fail_zst_result namespace_OpaqueZST_fail_zst(bool return_success);

    typedef struct namespace_OpaqueZST_success_fail_zst_result {union {somelib::ns::capi::RenamedOpaqueZST* ok; somelib::ns::capi::RenamedOpaqueZST* err;}; bool is_ok;} namespace_OpaqueZST_success_fail_zst_result;
    namespace_OpaqueZST_success_fail_zst_result namespace_OpaqueZST_success_fail_zst(bool return_success);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_optional_zst(bool is_some);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_static_getter(void);

    void namespace_OpaqueZST_static_setter(const somelib::ns::capi::RenamedOpaqueZST* _a);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_getter(const somelib::ns::capi::RenamedOpaqueZST* self);

    void namespace_OpaqueZST_setter(const somelib::ns::capi::RenamedOpaqueZST* self, const somelib::ns::capi::RenamedOpaqueZST* _a);

    somelib::ns::capi::RenamedOpaqueZSTIterator* namespace_OpaqueZST_iter(const somelib::ns::capi::RenamedOpaqueZST* self);

    somelib::ns::capi::RenamedOpaqueZST* namespace_OpaqueZST_indexer(const somelib::ns::capi::RenamedOpaqueZST* self, size_t _idx);

    void namespace_OpaqueZST_destroy(RenamedOpaqueZST* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::ctor() {
    auto result = somelib::ns::capi::namespace_OpaqueZST_ctor();
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::make() {
    auto result = somelib::ns::capi::namespace_OpaqueZST_make();
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::string somelib::ns::RenamedOpaqueZST::out_string() {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::ns::capi::namespace_OpaqueZST_out_string(&write);
    return output;
}
template<typename W>
inline void somelib::ns::RenamedOpaqueZST::out_string_write(W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::ns::capi::namespace_OpaqueZST_out_string(&write);
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::member() const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_member(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::mut_member() {
    auto result = somelib::ns::capi::namespace_OpaqueZST_mut_member(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::operator+(const somelib::ns::RenamedOpaqueZST& _o) const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_add(this->AsFFI(),
        _o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::operator-(const somelib::ns::RenamedOpaqueZST& _o) const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_sub(this->AsFFI(),
        _o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::operator*(const somelib::ns::RenamedOpaqueZST& _o) const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_mul(this->AsFFI(),
        _o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::operator/(const somelib::ns::RenamedOpaqueZST& _o) const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_div(this->AsFFI(),
        _o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::monostate> somelib::ns::RenamedOpaqueZST::success_zst(bool return_success) {
    auto result = somelib::ns::capi::namespace_OpaqueZST_success_zst(return_success);
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::monostate>(somelib::diplomat::Ok<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> somelib::ns::RenamedOpaqueZST::fail_zst(bool return_success) {
    auto result = somelib::ns::capi::namespace_OpaqueZST_fail_zst(return_success);
    return result.is_ok ? somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Err<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err))));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> somelib::ns::RenamedOpaqueZST::success_fail_zst(bool return_success) {
    auto result = somelib::ns::capi::namespace_OpaqueZST_success_fail_zst(return_success);
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Ok<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(somelib::diplomat::Err<std::unique_ptr<somelib::ns::RenamedOpaqueZST>>(std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result.err))));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::optional_zst(bool is_some) {
    auto result = somelib::ns::capi::namespace_OpaqueZST_optional_zst(is_some);
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::static_getter() {
    auto result = somelib::ns::capi::namespace_OpaqueZST_static_getter();
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline void somelib::ns::RenamedOpaqueZST::static_setter(const somelib::ns::RenamedOpaqueZST& _a) {
    somelib::ns::capi::namespace_OpaqueZST_static_setter(_a.AsFFI());
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::getter() const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_getter(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline void somelib::ns::RenamedOpaqueZST::setter(const somelib::ns::RenamedOpaqueZST& _a) const {
    somelib::ns::capi::namespace_OpaqueZST_setter(this->AsFFI(),
        _a.AsFFI());
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> somelib::ns::RenamedOpaqueZST::iter() const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_iter(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator>(somelib::ns::RenamedOpaqueZSTIterator::FromFFI(result));
}

inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueZSTIterator>somelib::ns::RenamedOpaqueZST::begin() const {
    return iter();
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> somelib::ns::RenamedOpaqueZST::operator[](size_t _idx) const {
    auto result = somelib::ns::capi::namespace_OpaqueZST_indexer(this->AsFFI(),
        _idx);
    return std::unique_ptr<somelib::ns::RenamedOpaqueZST>(somelib::ns::RenamedOpaqueZST::FromFFI(result));
}

inline const somelib::ns::capi::RenamedOpaqueZST* somelib::ns::RenamedOpaqueZST::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueZST*>(this);
}

inline somelib::ns::capi::RenamedOpaqueZST* somelib::ns::RenamedOpaqueZST::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueZST*>(this);
}

inline const somelib::ns::RenamedOpaqueZST* somelib::ns::RenamedOpaqueZST::FromFFI(const somelib::ns::capi::RenamedOpaqueZST* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueZST*>(ptr);
}

inline somelib::ns::RenamedOpaqueZST* somelib::ns::RenamedOpaqueZST::FromFFI(somelib::ns::capi::RenamedOpaqueZST* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueZST*>(ptr);
}

inline void somelib::ns::RenamedOpaqueZST::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueZST_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueZST*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueZST_HPP
