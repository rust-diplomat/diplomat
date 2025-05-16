import test from 'ava';
import { ErrorEnum, ErrorStruct, MyStruct, ResultOpaque } from 'diplomat-wasm-js-feature-tests';

test('Verify result methods', t => {
    const s = new ResultOpaque(5);
    s.assertInteger(5);

    const error1 = t.throws(() => ResultOpaque.newFailingFoo());
    t.is(error1.message, 'ErrorEnum.Foo');
    t.is(error1.cause, ErrorEnum.Foo);

    const error2 = t.throws(() => ResultOpaque.newFailingBar());
    t.is(error2.message, 'ErrorEnum.Bar');
    t.is(error2.cause, ErrorEnum.Bar);

    t.is(ResultOpaque.newFailingUnit(), null);

    const error3 = t.throws(() => ResultOpaque.newFailingStruct(109));
    t.is(error3.message, 'ErrorStruct: [object Object]')
    t.is((error3.cause as ErrorStruct).i, 109);

    const error4 = t.throws(() => ResultOpaque.newInErr(559));
    t.is(error4.message, 'ResultOpaque: [object Object]');
    (error4.cause as ResultOpaque).assertInteger(559);

    const error5 = t.throws(() => ResultOpaque.newInEnumErr(881));
    t.is(error5.message, 'ResultOpaque: [object Object]');
    (error5.cause as ResultOpaque).assertInteger(881);

    const error6 = t.throws(() => MyStruct.failsZstResult());
    t.is(error6.message, "MyZst");
});
