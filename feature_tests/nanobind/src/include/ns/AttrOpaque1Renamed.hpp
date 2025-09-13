#ifndef ns_AttrOpaque1Renamed_HPP
#define ns_AttrOpaque1Renamed_HPP

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


namespace ns {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result { bool is_ok;} DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result;

    typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t {
        const void* data;
        DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t;

    ns::capi::AttrOpaque1Renamed* namespace_AttrOpaque1_new(void);

    void namespace_AttrOpaque1_test_namespaced_callback(DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t _t_cb_wrap);

    int32_t namespace_AttrOpaque1_mac_test(void);

    int32_t namespace_AttrOpaque1_hello(void);

    uint8_t namespace_AttrOpaque1_method(const ns::capi::AttrOpaque1Renamed* self);

    uint8_t renamed_on_abi_only(const ns::capi::AttrOpaque1Renamed* self);

    void namespace_AttrOpaque1_use_unnamespaced(const ns::capi::AttrOpaque1Renamed* self, const diplomat::capi::Unnamespaced* _un);

    void namespace_AttrOpaque1_use_namespaced(const ns::capi::AttrOpaque1Renamed* self, ns::capi::RenamedAttrEnum _n);

    void namespace_AttrOpaque1_destroy(AttrOpaque1Renamed* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::AttrOpaque1Renamed> ns::AttrOpaque1Renamed::totally_not_new() {
    auto result = ns::capi::namespace_AttrOpaque1_new();
    return std::unique_ptr<ns::AttrOpaque1Renamed>(ns::AttrOpaque1Renamed::FromFFI(result));
}

inline void ns::AttrOpaque1Renamed::test_namespaced_callback(std::function<diplomat::result<std::monostate, std::monostate>()> _t) {
    ns::capi::namespace_AttrOpaque1_test_namespaced_callback({new decltype(_t)(std::move(_t)), diplomat::fn_traits(_t).template c_run_callback_result<std::monostate, std::monostate, ns::capi::DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result>, diplomat::fn_traits(_t).c_delete});
}

inline int32_t ns::AttrOpaque1Renamed::mac_test() {
    auto result = ns::capi::namespace_AttrOpaque1_mac_test();
    return result;
}

inline int32_t ns::AttrOpaque1Renamed::hello() {
    auto result = ns::capi::namespace_AttrOpaque1_hello();
    return result;
}

inline uint8_t ns::AttrOpaque1Renamed::method_renamed() const {
    auto result = ns::capi::namespace_AttrOpaque1_method(this->AsFFI());
    return result;
}

inline uint8_t ns::AttrOpaque1Renamed::abirenamed() const {
    auto result = ns::capi::renamed_on_abi_only(this->AsFFI());
    return result;
}

inline void ns::AttrOpaque1Renamed::use_unnamespaced(const Unnamespaced& _un) const {
    ns::capi::namespace_AttrOpaque1_use_unnamespaced(this->AsFFI(),
        _un.AsFFI());
}

inline void ns::AttrOpaque1Renamed::use_namespaced(ns::RenamedAttrEnum _n) const {
    ns::capi::namespace_AttrOpaque1_use_namespaced(this->AsFFI(),
        _n.AsFFI());
}

inline const ns::capi::AttrOpaque1Renamed* ns::AttrOpaque1Renamed::AsFFI() const {
    return reinterpret_cast<const ns::capi::AttrOpaque1Renamed*>(this);
}

inline ns::capi::AttrOpaque1Renamed* ns::AttrOpaque1Renamed::AsFFI() {
    return reinterpret_cast<ns::capi::AttrOpaque1Renamed*>(this);
}

inline const ns::AttrOpaque1Renamed* ns::AttrOpaque1Renamed::FromFFI(const ns::capi::AttrOpaque1Renamed* ptr) {
    return reinterpret_cast<const ns::AttrOpaque1Renamed*>(ptr);
}

inline ns::AttrOpaque1Renamed* ns::AttrOpaque1Renamed::FromFFI(ns::capi::AttrOpaque1Renamed* ptr) {
    return reinterpret_cast<ns::AttrOpaque1Renamed*>(ptr);
}

inline void ns::AttrOpaque1Renamed::operator delete(void* ptr) {
    ns::capi::namespace_AttrOpaque1_destroy(reinterpret_cast<ns::capi::AttrOpaque1Renamed*>(ptr));
}


#endif // ns_AttrOpaque1Renamed_HPP
