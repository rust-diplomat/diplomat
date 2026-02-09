#ifndef SOMELIB_ns_AttrOpaque1Renamed_HPP
#define SOMELIB_ns_AttrOpaque1Renamed_HPP

#include "AttrOpaque1Renamed.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../Unnamespaced.hpp"
#include "../diplomat_runtime.hpp"
#include "RenamedAttrEnum.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result { bool is_ok;} DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result;

    typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t {
        const void* data;
        DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t;

    void namespace_AttrOpaque1_test_namespaced_callback(DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t _t_cb_wrap);

    int32_t namespace_AttrOpaque1_mac_test(void);

    int32_t namespace_AttrOpaque1_hello(void);

    uint8_t namespace_AttrOpaque1_method(const somelib::ns::capi::AttrOpaque1Renamed* self);

    uint8_t renamed_on_abi_only(const somelib::ns::capi::AttrOpaque1Renamed* self);

    void namespace_AttrOpaque1_use_unnamespaced(const somelib::ns::capi::AttrOpaque1Renamed* self, const somelib::capi::Unnamespaced* _un);

    void namespace_AttrOpaque1_use_namespaced(const somelib::ns::capi::AttrOpaque1Renamed* self, somelib::ns::capi::RenamedAttrEnum _n);

    void namespace_AttrOpaque1_destroy(AttrOpaque1Renamed* self);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::ns::AttrOpaque1Renamed::test_namespaced_callback(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> _t) {
    somelib::ns::capi::namespace_AttrOpaque1_test_namespaced_callback({new decltype(_t)(std::move(_t)), somelib::diplomat::fn_traits(_t).template c_run_callback_result<std::monostate, std::monostate, somelib::ns::capi::DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result>, somelib::diplomat::fn_traits(_t).c_delete});
}

inline int32_t somelib::ns::AttrOpaque1Renamed::mac_test() {
    auto result = somelib::ns::capi::namespace_AttrOpaque1_mac_test();
    return result;
}

inline int32_t somelib::ns::AttrOpaque1Renamed::hello() {
    auto result = somelib::ns::capi::namespace_AttrOpaque1_hello();
    return result;
}

inline uint8_t somelib::ns::AttrOpaque1Renamed::method_renamed() const {
    auto result = somelib::ns::capi::namespace_AttrOpaque1_method(this->AsFFI());
    return result;
}

inline uint8_t somelib::ns::AttrOpaque1Renamed::abirenamed() const {
    auto result = somelib::ns::capi::renamed_on_abi_only(this->AsFFI());
    return result;
}

inline void somelib::ns::AttrOpaque1Renamed::use_unnamespaced(const somelib::Unnamespaced& _un) const {
    somelib::ns::capi::namespace_AttrOpaque1_use_unnamespaced(this->AsFFI(),
        _un.AsFFI());
}

inline void somelib::ns::AttrOpaque1Renamed::use_namespaced(somelib::ns::RenamedAttrEnum _n) const {
    somelib::ns::capi::namespace_AttrOpaque1_use_namespaced(this->AsFFI(),
        _n.AsFFI());
}

inline const somelib::ns::capi::AttrOpaque1Renamed* somelib::ns::AttrOpaque1Renamed::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::AttrOpaque1Renamed*>(this);
}

inline somelib::ns::capi::AttrOpaque1Renamed* somelib::ns::AttrOpaque1Renamed::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::AttrOpaque1Renamed*>(this);
}

inline const somelib::ns::AttrOpaque1Renamed* somelib::ns::AttrOpaque1Renamed::FromFFI(const somelib::ns::capi::AttrOpaque1Renamed* ptr) {
    return reinterpret_cast<const somelib::ns::AttrOpaque1Renamed*>(ptr);
}

inline somelib::ns::AttrOpaque1Renamed* somelib::ns::AttrOpaque1Renamed::FromFFI(somelib::ns::capi::AttrOpaque1Renamed* ptr) {
    return reinterpret_cast<somelib::ns::AttrOpaque1Renamed*>(ptr);
}

inline void somelib::ns::AttrOpaque1Renamed::operator delete(void* ptr) {
    somelib::ns::capi::namespace_AttrOpaque1_destroy(reinterpret_cast<somelib::ns::capi::AttrOpaque1Renamed*>(ptr));
}


#endif // SOMELIB_ns_AttrOpaque1Renamed_HPP
