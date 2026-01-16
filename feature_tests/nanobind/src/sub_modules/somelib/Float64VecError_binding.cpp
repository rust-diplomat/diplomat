#include "diplomat_nanobind_common.hpp"


#include "Float64VecError.hpp"

namespace somelib {
void add_Float64VecError_binding(nb::module_ mod) {
    PyType_Slot somelib_Float64VecError_slots[] = {
        {Py_tp_free, (void *)somelib::Float64VecError::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Float64VecError> opaque(mod, "Float64VecError", nb::type_slots(somelib_Float64VecError_slots));
    opaque
        .def("__getitem__", [](somelib::Float64VecError* self, size_t index) {
                auto out = self->operator[] (index);if (!out.is_ok()) {
                    auto errorPyV = nb::cast(std::move(std::move(out).err().value()));
                    if (errorPyV.is_valid())
                    {
                        throw nb::index_error(nb::str(errorPyV).c_str());
                    } else {
                        throw nb::index_error("Indexing error. Could not convert error type to string.");
                    }
                } else {
                    return out;
                }
            }, "i"_a)
        .def_static("new", &somelib::Float64VecError::new_, "v"_a);
}

} 