#include <iostream>
#include "../include/ns/AttrOpaque1Renamed.hpp"
#include "../include/ns/RenamedAttrEnum.hpp"
#include "../include/Unnamespaced.hpp"
#include "../include/nested/ns/Nested.hpp"
#include "../include/nested/ns2/Nested.hpp"
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
}
