<?php
namespace TraitFlagInputs;
trait BoundFlags { /** @var int */ public const FLAGS = self::ZERO ?: parent::TARGET; }
class ClassSeed { public const TARGET = \Attribute::TARGET_CLASS; }
class MethodSeed { public const TARGET = \Attribute::TARGET_METHOD; }
#[\Attribute(self::FLAGS)]
class ClassAttribute extends ClassSeed { use BoundFlags; public const ZERO = 0; }
#[\Attribute(self::FLAGS)]
class MethodAttribute extends MethodSeed { use BoundFlags; public const ZERO = 0; }
#[ClassAttribute] class ClassUse {}
#[MethodAttribute] class MethodUse {}
trait CoalesceFlags { public const FLAGS = self::ZERO ?? parent::TARGET; }
#[\Attribute(self::FLAGS)]
class CoalesceAttribute extends ClassSeed { use CoalesceFlags; public const ZERO = 0; }
#[CoalesceAttribute] class CoalesceUse {}
trait LongFlags { public const FLAGS = true ? self::TARGET : parent::TARGET; }
#[\Attribute(self::FLAGS)]
class SameAttribute extends ClassSeed { use LongFlags; }
#[\Attribute(self::FLAGS)]
class UnionAttribute extends MethodSeed { use LongFlags; public const TARGET = 1; }
#[SameAttribute] class SameUse {}
#[UnionAttribute] class UnionUse {}
#[\Attribute(self::FLAGS)]
class ScannedAttribute extends ClassSeed { use ScannedFlags; }
#[ScannedAttribute] class ScannedUse {}
