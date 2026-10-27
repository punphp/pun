<?php

namespace N;

$a = <<<'X'
Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass
X;
$b = <<<X
Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass
X;
$c = "Rector\\Tests\\Php55\\Rector\\String_\\StringClassNameToClassConstantRector\\Source\\AnotherClass";
$d = \is_a($x, 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass');
$e = IS_A($x, 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass');
$f = foo('Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass') . 'x';
$g = 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\anotherclass';
$h = 'rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass';
$i = ['Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\SomeUser' => 1];
$j = 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass'::FOO;
$k = 'Nette\Utils\Strings';
$l = 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\View';
-----
<?php

namespace N;

$a = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class;
$b = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class;
$c = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class;
$d = \is_a($x, 'Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass');
$e = IS_A($x, \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class);
$f = foo(\Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class) . 'x';
$g = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\anotherclass::class;
$h = 'rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass';
$i = [\Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\SomeUser::class => 1];
$j = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\AnotherClass::class::FOO;
$k = 'Nette\Utils\Strings';
$l = \Rector\Tests\Php55\Rector\String_\StringClassNameToClassConstantRector\Source\View::class;
