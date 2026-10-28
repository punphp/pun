<?php
// php closure-matcher.php /absolute/path/to/phpstan-2.3.1.phar > closure-matcher.tsv
require 'phar://' . $argv[1] . '/vendor/autoload.php';
use PHPStan\Analyser\ClosureCallContextMatcher as Matcher;
use PHPStan\Reflection\Dummy\DummyMethodReflection;
use PHPStan\Reflection\Native\NativeFunctionReflection;
use PHPStan\Reflection\Php\DummyParameter;
use PHPStan\TrinaryLogic;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntegerType;
use PHPStan\Type\MixedType;
use PHPStan\Type\StringType;
$method = new DummyMethodReflection('read');
$function = new NativeFunctionReflection('map', [], null, null, TrinaryLogic::createMaybe(), false, null, null, null, true, []);
$parameter = static fn($type) => new DummyParameter('callback', $type, false, null, false, null);
$template = static fn($scope) => TemplateTypeFactory::create($scope, 'T', new MixedType(), TemplateTypeVariance::createInvariant());
$int = $parameter(new IntegerType());
$string = $parameter(new StringType());
$cases = [
    'no-asking-parameter' => [[$method, null], [$method, $int]],
    'equal-full-type' => [[$function, $parameter(new IntegerType())], [$method, $int]],
    'no-position-callee' => [[null, $string], [null, $int]],
    'no-position-parameter' => [[$method, $string], [$method, null]],
    'own-method-template' => [[$method, $parameter($template(TemplateTypeScope::createWithMethod('Other', 'READ')))], [$method, $int]],
    'own-function-template' => [[$function, $parameter($template(TemplateTypeScope::createWithFunction('MAP')))], [$function, $int]],
    'foreign-function-template' => [[$method, $parameter($template(TemplateTypeScope::createWithFunction('read')))], [$method, $int]],
    'foreign-method-template' => [[$method, $parameter($template(TemplateTypeScope::createWithMethod('Other', 'different')))], [$method, $int]],
    'class-template' => [[$method, $parameter($template(TemplateTypeScope::createWithClass('Other')))], [$method, $int]],
];
foreach ($cases as $name => [$ask, $position]) {
    echo $name, "\t", Matcher::matches([$ask], [$position]) ? '1' : '0', "\n";
}
