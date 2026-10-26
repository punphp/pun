<?php
namespace ScannedHolders;
class A { public const V = GlobalValues::V; }
class B { public const V = 4; }
class GlobalValues { public const V = true ? 1 : 2; }
const HOLDERS = true ? A::class : B::class;
const REFLECTED = (HOLDERS)::V;
class Values {
    public const HOLDERS = true ? A::class : B::class;
    public const VALUE = (self::HOLDERS)::V;
}
#[\Attribute((true ? TargetA::class : TargetB::class)::FLAGS)] class EitherClass {}
class TargetA { public const FLAGS = \Attribute::TARGET_CLASS; }
class TargetB { public const FLAGS = \Attribute::TARGET_CLASS; }
#[\Attribute((true ? TargetA::class : DifferentTarget::class)::FLAGS)] class DifferentTargets {}
class DifferentTarget { public const FLAGS = \Attribute::TARGET_METHOD; }
