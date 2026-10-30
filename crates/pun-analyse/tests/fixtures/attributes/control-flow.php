<?php

namespace AttributeControlFlow;

const CLASS_FLAG = 1;
const METHOD_FLAG = 4;

class Flags
{
    public const CLASS_FLAG = 1;
}

#[\Attribute(false ? 1 : 1)]
class SameBranches {}

#[\Attribute(true ? 1 : 4)]
class DistinctBranches {}

#[\Attribute(false ? 4 : 1)]
class ReversedBranches {}

#[\Attribute(true ? 0 : 1)]
class FalseyThen {}

#[\Attribute(true ? false : 1)]
class FalseThen {}

#[\Attribute(false ? null : 1)]
class NullThen {}

#[\Attribute(true ? '' : 1)]
class EmptyStringThen {}

#[\Attribute(true ? '0' : 1)]
class ZeroStringThen {}

#[\Attribute(true ? [] : 1)]
class EmptyArrayThen {}

#[\Attribute(true ? [1] : 1)]
class NonEmptyArrayThen {}

#[\Attribute(false ? 1 : 0)]
class FalseyElse {}

#[\Attribute(true ? 0 : 0)]
class ZeroBranches {}

#[\Attribute(new \stdClass ? 1 : 1)]
class UnusedCondition {}

#[SameBranches, FalseyThen, FalseThen, NullThen, EmptyStringThen, ZeroStringThen, EmptyArrayThen, UnusedCondition]
class LongSupported {}

#[DistinctBranches, ReversedBranches, FalseyElse, NonEmptyArrayThen]
class LongUnions {}

#[ZeroBranches]
class LongZero {}

#[SameBranches]
function wrongLongTarget(): void {}

#[\Attribute(0 ?: 1)]
class ShortZero {}

#[\Attribute(false ?: 1)]
class ShortFalse {}

#[\Attribute(null ?: 1)]
class ShortNull {}

#[\Attribute('' ?: 1)]
class ShortEmptyString {}

#[\Attribute('0' ?: 1)]
class ShortZeroString {}

#[\Attribute([] ?: 1)]
class ShortEmptyArray {}

#[\Attribute([1] ?: 1)]
class ShortNonEmptyArray {}

#[\Attribute(1 ?: 1)]
class ShortSame {}

#[\Attribute(1 ?: 4)]
class ShortDistinct {}

#[\Attribute('flags' ?: 1)]
class ShortTruthyString {}

#[\Attribute(0 ?: 0)]
class ShortNoTargets {}

#[ShortZero, ShortFalse, ShortNull, ShortEmptyString, ShortZeroString, ShortEmptyArray, ShortSame]
class ShortSupported {}

#[ShortDistinct, ShortTruthyString, ShortNonEmptyArray]
class ShortUnions {}

#[ShortNoTargets]
class ShortZeroUse {}

#[\Attribute(null ?? 1)]
class CoalesceNull {}

#[\Attribute(1 ?? 1)]
class CoalesceSame {}

#[\Attribute(0 ?? 1)]
class CoalesceDistinct {}

#[\Attribute(false ?? 1)]
class CoalesceFalse {}

#[\Attribute('' ?? 1)]
class CoalesceEmptyString {}

#[\Attribute('0' ?? 1)]
class CoalesceZeroString {}

#[\Attribute([] ?? 1)]
class CoalesceEmptyArray {}

#[\Attribute(0 ?? 0)]
class CoalesceNoTargets {}

#[\Attribute(1 ?? null)]
class CoalesceNullRight {}

#[CoalesceNull, CoalesceSame]
class CoalesceSupported {}

#[CoalesceDistinct, CoalesceFalse, CoalesceEmptyString, CoalesceZeroString, CoalesceEmptyArray, CoalesceNullRight]
class CoalesceUnions {}

#[CoalesceNoTargets]
class CoalesceZeroUse {}

#[\Attribute((((null ?? (1))) | ((0 ?: (4))) | (128)))]
class NestedMask {}

#[NestedMask, NestedMask]
class RepeatableMask
{
    #[NestedMask]
    public function supported(): void {}

    #[NestedMask]
    public int $unsupported = 0;
}

#[\Attribute(false ? \Attribute::TARGET_CLASS : \Attribute::TARGET_CLASS)]
class BuiltinReferences {}

#[\Attribute(true ? CLASS_FLAG : CLASS_FLAG)]
class SameReferences {}

#[\Attribute(false ? CLASS_FLAG : METHOD_FLAG)]
class DistinctReferences {}

#[\Attribute(null ?? Flags::CLASS_FLAG)]
class ClassReference {}

#[\Attribute(0 ?: CLASS_FLAG)]
class GlobalReference {}

#[BuiltinReferences, SameReferences, ClassReference, GlobalReference]
class ReferenceSupported {}

#[DistinctReferences]
class ReferenceUnion {}

#[\Attribute(true ? new \stdClass : 1)]
class UnsupportedThen {}

#[\Attribute(true ? 1 : new \stdClass)]
class UnsupportedElse {}

#[\Attribute(new \stdClass ?? 1)]
class UnsupportedLeft {}

#[\Attribute(null ?? new \stdClass)]
class UnsupportedRight {}

#[UnsupportedThen, UnsupportedElse, UnsupportedLeft, UnsupportedRight]
class UnsupportedOperands {}
