<?php
namespace FiniteHolders;
class A { public const V = 1; public const int TYPED = 1; /** @var positive-int */ public const DOC = 1; }
class B { public const V = 4; public const int TYPED = 4; /** @var positive-int */ public const DOC = 4; }
class EmptyA {}
class EmptyB {}
class Values {
    public const HOLDERS = true ? A::class : B::class;
    public const TWO = (true ? A::class : B::class)::V;
    public const FALSE_CONDITION = (false ? A::class : B::class)::V;
    public const SAME = (true ? A::class : A::class)::V;
    public const ALIAS = (self::HOLDERS)::V;
    public const TYPED = (true ? A::class : B::class)::TYPED;
    public const DOC = (true ? A::class : B::class)::DOC;
    public const UNKNOWN_LITERAL = (true ? A::class : 'NoSuchFiniteHolder')::V;
    public const UNKNOWN_MAGIC = (true ? A::class : NoSuchFiniteHolder::class)::V;
    public const ONE_MISSING_MEMBER = (true ? A::class : EmptyA::class)::V;
    public const ALL_MISSING_MEMBERS = (true ? EmptyA::class : EmptyB::class)::V;
    public const INTEGER_ARM = (true ? A::class : 0)::V;
    public const NULL_ARM = (true ? A::class : null)::V;
    public const CALL_ARM = (true ? A::class : strlen('x'))::V;
    public const COALESCE = (null ?? A::class)::V;
    public const CLASS_NAMES = (true ? A::class : B::class)::class;
    public const CONCAT = ('FiniteHolders\\' . 'A')::V;
    public const DYNAMIC_NAME = A::{'V'};
    public const NAMED = A::V;
}
\PHPStan\dumpType(Values::TWO);
\PHPStan\dumpType(Values::FALSE_CONDITION);
\PHPStan\dumpType(Values::SAME);
\PHPStan\dumpType(Values::ALIAS);
\PHPStan\dumpType(Values::TYPED);
\PHPStan\dumpType(Values::DOC);
\PHPStan\dumpType(Values::UNKNOWN_LITERAL);
\PHPStan\dumpType(Values::UNKNOWN_MAGIC);
\PHPStan\dumpType(Values::ONE_MISSING_MEMBER);
\PHPStan\dumpType(Values::ALL_MISSING_MEMBERS);
\PHPStan\dumpType(Values::INTEGER_ARM);
\PHPStan\dumpType(Values::NULL_ARM);
\PHPStan\dumpType(Values::CALL_ARM);
\PHPStan\dumpType(Values::COALESCE);
\PHPStan\dumpType(Values::CLASS_NAMES);
\PHPStan\dumpType(Values::CONCAT);
\PHPStan\dumpType(Values::DYNAMIC_NAME);
\PHPStan\dumpType(Values::NAMED);
\PHPStan\dumpType(GLOBAL_VALUE);
const GLOBAL_VALUE = (true ? A::class : B::class)::V;
\PHPStan\dumpType(GLOBAL_VALUE);
function ordinary(bool $condition): void {
    \PHPStan\dumpType((true ? A::class : B::class)::V);
    \PHPStan\dumpType(($condition ? A::class : B::class)::V);
    \PHPStan\dumpType(A::TYPED);
    \PHPStan\dumpType(A::DOC);
}
trait UnionSource { public const VALUE = (true ? A::class : B::class)::V; }
class EqualUnion { use UnionSource; public const VALUE = true ? 1 : 4; }
class DifferentUnion { use UnionSource; public const VALUE = 1; }
