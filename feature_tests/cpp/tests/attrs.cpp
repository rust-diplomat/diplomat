#include <iostream>
#include "../include/ns/AttrOpaque1Renamed.hpp"
#include "../include/ns/RenamedOpaqueArithmetic.hpp"
#include "../include/ns/RenamedAttrEnum.hpp"
#include "../include/Unnamespaced.hpp"
#include "../include/nested/ns/Nested.hpp"
#include "../include/nested/ns2/Nested.hpp"
#include "../include/Float64Vec.hpp"

#include "assert.hpp"

int main(int argc, char *argv[])
{
    std::unique_ptr<ns::AttrOpaque1Renamed> r = ns::AttrOpaque1Renamed::totally_not_new();
    simple_assert_eq("method should call", r->method_renamed(), 77);
    simple_assert_eq("method should call", r->abirenamed(), 123);

    // These C names should also resolve
    void *renamed = (void *)ns::capi::renamed_on_abi_only;
    std::cout << "Renamed function at " << renamed << std::endl;
    renamed = (void *)ns::capi::namespace_AttrOpaque1_method;
    std::cout << "Renamed function at " << renamed << std::endl;

    ns::RenamedAttrEnum e = ns::RenamedAttrEnum::A;

    std::unique_ptr<Unnamespaced> un = Unnamespaced::make(e);
    un->use_namespaced(*r);
    r->use_unnamespaced(*un);
    r->use_namespaced(e);

    auto a = ns::RenamedOpaqueArithmetic::make(1, 2);
    auto b = ns::RenamedOpaqueArithmetic::make(2, 3);
    {
        auto r = (*a) + (*b);

        simple_assert_eq("adding x", r->x(), 3);
        simple_assert_eq("adding y", r->y(), 5);
    }

    auto array = std::array{1.5, 1.6};
    auto vec = Float64Vec::new_(array);
    simple_assert_eq("vector indexer", (*vec)[0].value(), 1.5);
    simple_assert_eq("vector indexer", (*vec)[1].value(), 1.6);
    simple_assert_eq("vector indexer", (*vec)[2].has_value(), false);
}
