<?php
$a = 'DateTime';
$b = '\DateTime';
$c = "ArrayObject";
$d = 'datetime';
$e = 'Countable';
$f = 'BackedEnum';
$g = 'Nope\Missing';
$h = 'Foo' . 'Exception';
$i = 'Exception';
$j = '\\Exception';
$k = 'Exception ';
$l = "Exception$x";
$m = <<<'X'
Exception
X;
$n = '';
$o = '\\';
$p = 'JsonException';
$q = 'Traversable';
echo 'DateTime', "Stringable";
$s = 'java';
$t = 'ZendAPI_Job';
-----
<?php
$a = \DateTime::class;
$b = \DateTime::class;
$c = \ArrayObject::class;
$d = 'datetime';
$e = \Countable::class;
$f = \BackedEnum::class;
$g = 'Nope\Missing';
$h = 'Foo' . \Exception::class;
$i = \Exception::class;
$j = \Exception::class;
$k = 'Exception ';
$l = "Exception$x";
$m = <<<'X'
Exception
X;
$n = '';
$o = '\\';
$p = \JsonException::class;
$q = \Traversable::class;
echo \DateTime::class, \Stringable::class;
$s = 'java';
$t = 'ZendAPI_Job';
