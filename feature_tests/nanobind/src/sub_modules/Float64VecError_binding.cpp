#include "diplomat_nanobind_common.hpp"


#include "Float64VecError.hpp"


void add_Float64VecError_binding(nb::module_ mod) {
    PyType_Slot Float64VecError_slots[] = {
        {Py_tp_free, (void *)Float64VecError::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Float64VecError>(mod, "Float64VecError", nb::type_slots(Float64VecError_slots))
        .def("__getitem__", [](Float64VecError* self, size_t index) {
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
        .def_static("new", &Float64VecError::new_, "v"_a);
}

