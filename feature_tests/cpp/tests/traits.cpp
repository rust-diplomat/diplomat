#include "../include/TraitWrapper.hpp"
#include "../include/TesterTrait.hpp"
#include "assert.hpp"



using namespace somelib;

int main(int argc, char* argv[]) {
    class ImplementsTrait : public TesterTrait {
        protected:
        virtual uint32_t test_trait_fn(uint32_t x) override {
            return x + 5;
        }

        virtual void test_void_trait_fn() override {
            std::cout << "Void trait!" << std::endl;
        }

        virtual int32_t test_struct_trait_fn(somelib::TraitTestingStruct s) override {
            return s.x;
        }

        virtual somelib::diplomat::result<uint32_t, std::monostate> test_result_output() override {
            return somelib::diplomat::Ok<uint32_t>(0);
        }

        virtual std::optional<uint32_t> test_optional_output(uint32_t x) {
            return std::optional(x);
        }

        virtual somelib::diplomat::result<uint32_t, std::optional<uint32_t>> test_result_of_optional(bool is_ok) override {
            if (is_ok) {
                return somelib::diplomat::Ok<uint32_t>(5);
            } else {
                return somelib::diplomat::Err<std::optional<uint32_t>>(std::optional(10));
            }
        }
    };

    auto trait_test = new ImplementsTrait();
    // Do not insert trait calls through simple_assert_eq, unique_ptr will be called twice if there is no equivalence:
    auto pass_result = TraitWrapper::test_with_trait(std::unique_ptr<TesterTrait>(trait_test), 100);
    simple_assert_eq("Trait passing", pass_result, 105);

    trait_test = new ImplementsTrait();

    auto struct_convert_res = TraitWrapper::test_trait_with_struct(std::unique_ptr<TesterTrait>(trait_test));
    simple_assert_eq("Trait struct convert", struct_convert_res, 1);

    trait_test = new ImplementsTrait();
    TraitWrapper::test_result_output(std::unique_ptr<TesterTrait>(trait_test));

    trait_test = new ImplementsTrait();
    TraitWrapper::test_optional_output(std::unique_ptr<TesterTrait>(trait_test), 5);

    trait_test = new ImplementsTrait();
    auto ok = TraitWrapper::test_result_of_optional(std::unique_ptr<TesterTrait>(trait_test), true);
    simple_assert_eq("Trait result return", std::move(ok).ok().value(), 5);

    trait_test = new ImplementsTrait();
    auto err = TraitWrapper::test_result_of_optional(std::unique_ptr<TesterTrait>(trait_test), false);
    simple_assert_eq("Trait result return err", std::move(err).err().value().value(), 10);
}