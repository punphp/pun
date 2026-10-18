<?php

namespace AttributeFlags;

const T = 4;

#[\Attribute(T)]
class GlobalConstant {}

#[\Attribute(self::X)]
class ClassConstant
{
    const X = Other::Y;
}

class Other
{
    const Y = 4;
}

#[\Attribute(1 + 3)]
class Sum {}

#[\Attribute('x')]
class NotAnInteger {}

#[\Attribute]
#[\Attribute]
class Twice {}

#[\attribute]
class Lowercase {}

#[\Attribute(\PHP_INT_SIZE)]
class PlatformConstant {}

#[\Attribute(flags: \Attribute::TARGET_METHOD)]
class Named {}

#[\Attribute(foo: \Attribute::TARGET_CLASS)]
class UnknownNamed {}

#[\Attribute(256)]
class Big {}

#[GlobalConstant, ClassConstant, Sum, NotAnInteger, Twice, Lowercase, Named, UnknownNamed, \Cassandra\Bigint, Big, Big]
class Uses
{
    #[PlatformConstant]
    public int $p = 1;
}

$x = new #[Named] class {};

$y = new #[\AllowDynamicProperties] readonly class {};

#[\SensitiveParameter]
function builtin(): void {}
