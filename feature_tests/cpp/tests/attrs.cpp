#include <iostream>
#include "../include/ns/AttrOpaque1Renamed.hpp"
#include "../include/ns/RenamedOpaqueArithmetic.hpp"
#include "../include/ns/RenamedAttrEnum.hpp"
#include "../include/ns/RenamedMyIterable.hpp"
#include "../include/ns/RenamedOpaqueIterable.hpp"
#include "../include/ns/RenamedOpaqueRefIterable.hpp"
#include "../include/ns/RenamedComparable.hpp"
#include "../include/ns/RenamedVectorTest.hpp"
#include "../include/Unnamespaced.hpp"
#include "../include/nested/ns/Nested.hpp"
#include "../include/nested/ns2/Nested.hpp"
#include "../include/Float64Vec.hpp"
#include "../include/ns/RenamedStringList.hpp"
#include "../include/ns/RenamedBlockOverride.hpp"

using namespace somelib;

#include "assert.hpp"

int main(int argc, char* argv[]) {
    std::unique_ptr<ns::AttrOpaque1Renamed> r = ns::AttrOpaque1Renamed::totally_not_new();
    simple_assert_eq("method should call", r->method_renamed(), 77);
    simple_assert_eq("method should call", r->abirenamed(), 123);
    simple_assert_eq("Macro method should call", r->mac_test(), 10);
    simple_assert_eq("Macro method should call", r->hello(), 0);

    // These C names should also resolve
    void* renamed = (void*)ns::capi::renamed_on_abi_only;
    std::cout << "Renamed function at " << renamed << std::endl;
    renamed = (void*)ns::capi::namespace_AttrOpaque1_method;
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

    auto array = std::array{ 1.5, 1.6 };
    auto vec = Float64Vec::new_(array);
    simple_assert_eq("vector indexer", (*vec)[0].value(), 1.5);
    simple_assert_eq("vector indexer", (*vec)[1].value(), 1.6);
    simple_assert_eq("vector indexer", (*vec)[2].has_value(), false);


    // Iterators returning std::optional types
    {
        auto uintVec = std::vector<uint8_t>{ 1, 2, 3, 4 };
        auto myIterable = ns::RenamedMyIterable::new_(diplomat::span<const uint8_t>{uintVec.data(), uintVec.size()});
        auto myIt = myIterable->begin();

        simple_assert_eq("Iteration dereference", *myIt, 1);
        myIt++;
        simple_assert_eq("Iteration manual increment", *myIt, 2);

        auto unitVecCopy = std::vector<uint8_t>();
        for (auto element : *myIterable) {
            unitVecCopy.push_back(element);
        }
        simple_assert("For loop iteration", uintVec == unitVecCopy);
        simple_assert("stl-algorithm iteration failed", std::equal(uintVec.begin(), uintVec.end(), myIterable->begin()));
    }

    // Iterators returning std::unique_ptr opaque types
    {
        auto myOpaqueIterable = ns::RenamedOpaqueIterable::new_(2);
        size_t count = 0;
        for (auto& element : *myOpaqueIterable) {
            simple_assert("Opaque type access", element.method_renamed() == 77);
            count++;
        }
        simple_assert("For loop iteration count", count == 2);
    }

    // Iterators returning references (pointers internally) to opaque types
    {
        auto myOpaqueRefIterable = ns::RenamedOpaqueRefIterable::new_(2);
        size_t count = 0;
        for (auto& element : *myOpaqueRefIterable) {
            simple_assert("Opaque type access", element.method_renamed() == 77);
            count++;
        }
        simple_assert("For loop iteration count", count == 2);
    }

    auto cmpA = ns::RenamedComparable::new_(0);
    auto cmpB = ns::RenamedComparable::new_(0);
    auto cmpC = ns::RenamedComparable::new_(1);
    simple_assert("equality", *cmpA == *cmpB);
    simple_assert("nequality", *cmpB != *cmpC);
    simple_assert("less or equal as equals", *cmpA <= *cmpB);
    simple_assert("greater or equal as equals", *cmpA >= *cmpB);
    simple_assert("less or equal", *cmpA <= *cmpC);
    simple_assert("greater or equal", *cmpC >= *cmpA);
    simple_assert("less", *cmpA < *cmpC);
    simple_assert("greater", *cmpC > *cmpA);

    auto v = ns::RenamedVectorTest::new_();
    v->push(0.0f);
    v->push(1.0f);
    v->push(2.0f);
    simple_assert_eq("Macro vector indexing", (*v)[0].value(), 0.0f);
    simple_assert_eq("Macro vector indexing 2", (*v)[2].value(), 2.0f);

    std::vector<std::string> vec_bound = ns::RenamedStringList::return_new();
    simple_assert_eq("Custom bindings", vec_bound[0], "Test!");

    simple_assert_eq("Custom block bindings", ns::RenamedBlockOverride::special_function(), "This is a custom binding.");
    simple_assert_eq("Custom block bindings", ns::RenamedBlockOverride::custom_bool, false);
}
