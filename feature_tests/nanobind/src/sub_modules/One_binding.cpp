#include "diplomat_nanobind_common.hpp"


#include "One.hpp"
#include "Two.hpp"


void add_One_binding(nb::module_ mod) {
    PyType_Slot One_slots[] = {
        {Py_tp_free, (void *)One::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<One>(mod, "One", nb::type_slots(One_slots))
        .def_static("cycle", &One::cycle, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
        .def_static("diamond_and_nested_types", &One::diamond_and_nested_types, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("diamond_bottom", &One::diamond_bottom, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("diamond_left", &One::diamond_left, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 2>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("diamond_right", &One::diamond_right, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("diamond_top", &One::diamond_top, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("implicit_bounds", &One::implicit_bounds, "explicit_hold"_a, "implicit_hold"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>() ) // unsupported special method NamedConstructor(None)
        .def_static("implicit_bounds_deep", &One::implicit_bounds_deep, "explicit_"_a, "implicit_1"_a, "implicit_2"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>() ) // unsupported special method NamedConstructor(None)
        .def_static("many_dependents", &One::many_dependents, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
        .def_static("return_outlives_param", &One::return_outlives_param, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
        .def_static("transitivity", &One::transitivity, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    ;
}

