<?php
namespace App;
use function Foo\bar;
use function Foo\{baz as q};
bar(); Bar(); q(); baz(); \strlen('a'); Sub\f(); namespace\g(); strlen('a'); $f(); exit(); die(); isset($a); empty($a); compact('a'); extract($a); func_get_args();
