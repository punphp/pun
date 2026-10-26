<?php

namespace AttributeInputFlags;

#[\Attribute((((1))))]
class Scalar {}

#[Scalar]
class SupportedClass {}

#[Scalar]
function unsupportedFunction(): void {}

#[\Attribute((((1) | ((4))) | (128)))]
class Bitmask {}

#[Bitmask, Bitmask]
class RepeatedClass
{
    #[Bitmask]
    public function supported(): void {}

    #[Bitmask]
    public int $unsupported = 0;
}

#[\Attribute(((\Attribute::TARGET_FUNCTION)))]
class Constant {}

#[Constant]
function supportedFunction(): void {}

#[Constant]
class UnsupportedClass {}

#[\Attribute(((0)))]
class NoTargets {}

#[NoTargets]
class UnsupportedZero {}

#[\Attribute((('1')))]
class StringFlags {}

#[\Attribute(((1.0)))]
class FloatFlags {}

#[\Attribute(((true)))]
class BooleanFlags {}

#[\Attribute(((null)))]
class NullFlags {}

#[\Attribute(((MISSING_FLAGS)))]
class UnresolvedFlags {}

#[StringFlags, FloatFlags, BooleanFlags, NullFlags, UnresolvedFlags]
class InvalidFlags {}
