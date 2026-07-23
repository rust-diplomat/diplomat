#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueZST.hpp"

namespace somelib::ns {
void add_RenamedOpaqueZST_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueZST> opaque(mod, "RenamedOpaqueZST", "Tests for https://github.com/rust-diplomat/diplomat/issues/1050.\nC++ generates unique_ptrs for Opaque ZSTs, and Nanobind\nexpects every unique_ptr it converts to wrap a unique pointer type. It errors otherwise.\nThis is not the case, as in Rust pointers to ZSTs are always the same address.");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::ctor))))
        .def("__add__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::operator+)), nb::is_operator())
        .def("__truediv__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::operator/)), nb::is_operator())
        .def_static("fail_zst", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::fail_zst)), "return_success"_a)
        .def_prop_rw("getter", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::getter)), &somelib::ns::RenamedOpaqueZST::setter)
        .def("__getitem__", [](somelib::ns::RenamedOpaqueZST* self, size_t index) {
                auto out = map_inner(self->operator[] (index));
                if (out == nullptr) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }}, "_idx"_a)
        .def("__iter__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::iter)))
        .def_static("make", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::make)))
        .def("member", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::member)))
        .def("__mul__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::operator*)), nb::is_operator())
        .def("mut_member", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::mut_member)))
        .def_static("optional_zst", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::optional_zst)), "is_some"_a)
        .def_prop_ro_static("out_string", [](nb::handle) -> decltype(somelib::ns::RenamedOpaqueZST::out_string()) { return somelib::ns::RenamedOpaqueZST::out_string(); })
        .def_prop_rw_static("static_getter", [](nb::handle) -> decltype(somelib::ns::RenamedOpaqueZST::static_getter()) { return map_inner(somelib::ns::RenamedOpaqueZST::static_getter()); },
                    [](nb::handle, const somelib::ns::RenamedOpaqueZST& _a)
                      { somelib::ns::RenamedOpaqueZST::static_setter(_a); })
        .def("__sub__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::operator-)), nb::is_operator())
        .def_static("success_fail_zst", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::success_fail_zst)), "return_success"_a)
        .def_static("success_zst", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZST::success_zst)), "return_success"_a);
}

} 