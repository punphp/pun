<?php
namespace HolderPolicies;
class A {
    public const V = 1;
    public const int TYPED = 1;
    /** @var positive-int */ public const DOC = 1;
    final public const STABLE = 1;
    final public const int STABLE_TYPED = 1;
}
class B { public const V = 4; public const int TYPED = 4; }
final class FinalA { public const V = 1; public const int TYPED = 1; /** @var positive-int */ public const DOC = 1; }
final class FinalB { public const V = 4; public const int TYPED = 4; }
class Values {
    public const OBJECT = (new A)::V;
    public const OBJECT_UNION = (true ? new A : new B)::V;
    public const OBJECT_TYPED = (new A)::TYPED;
    public const OBJECT_DOC = (new A)::DOC;
    public const OBJECT_STABLE = (new A)::STABLE;
    public const OBJECT_STABLE_TYPED = (new A)::STABLE_TYPED;
    public const FINAL_UNION = (true ? new FinalA : new FinalB)::V;
    public const FINAL_TYPED = (new FinalA)::TYPED;
    public const FINAL_DOC = (new FinalA)::DOC;
    public const STRING_TYPED = (A::class)::TYPED;
    public const STRING_DOC = (A::class)::DOC;
    public const OBJECT_UNION_TYPED = (true ? new A : new B)::TYPED;
    public const OBJECT_UNKNOWN = (true ? new FinalA : new NoSuchPolicyHolder)::V;
    public const OBJECT_STRING = (true ? new FinalA : FinalB::class)::V;
    public const OBJECT_INTEGER = (true ? new FinalA : 0)::V;
    public const OBJECT_CALL = (true ? new FinalA : strlen('x'))::V;
    public const DYNAMIC_OBJECT = (new (A::class))::V;
}
\PHPStan\dumpType(Values::OBJECT);
\PHPStan\dumpType(Values::OBJECT_UNION);
\PHPStan\dumpType(Values::OBJECT_TYPED);
\PHPStan\dumpType(Values::OBJECT_DOC);
\PHPStan\dumpType(Values::OBJECT_STABLE);
\PHPStan\dumpType(Values::OBJECT_STABLE_TYPED);
\PHPStan\dumpType(Values::FINAL_UNION);
\PHPStan\dumpType(Values::FINAL_TYPED);
\PHPStan\dumpType(Values::FINAL_DOC);
\PHPStan\dumpType(Values::STRING_TYPED);
\PHPStan\dumpType(Values::STRING_DOC);
\PHPStan\dumpType(Values::OBJECT_UNION_TYPED);
\PHPStan\dumpType(Values::OBJECT_UNKNOWN);
\PHPStan\dumpType(Values::OBJECT_STRING);
\PHPStan\dumpType(Values::OBJECT_INTEGER);
\PHPStan\dumpType(Values::OBJECT_CALL);
\PHPStan\dumpType(Values::DYNAMIC_OBJECT);
class ParentClass { public const V = 7; public const int TYPED = 7; }
class Relative extends ParentClass {
    public const V = 3;
    public const int TYPED = 3;
    public const SELF_VALUE = self::V;
    public const PARENT_VALUE = parent::V;
    public const STATIC_VALUE = static::V;
    public const STATIC_TYPED = static::TYPED;
    public const SELF_PARENT = (true ? self::class : parent::class)::V;
    public const STATIC_HOLDER = (static::class)::V;
}
final class FinalRelative extends ParentClass {
    public const V = 5;
    public const STATIC_VALUE = static::V;
    public const STATIC_HOLDER = (static::class)::V;
}
\PHPStan\dumpType(Relative::SELF_VALUE);
\PHPStan\dumpType(Relative::PARENT_VALUE);
\PHPStan\dumpType(Relative::STATIC_VALUE);
\PHPStan\dumpType(Relative::STATIC_TYPED);
\PHPStan\dumpType(Relative::SELF_PARENT);
\PHPStan\dumpType(Relative::STATIC_HOLDER);
\PHPStan\dumpType(FinalRelative::STATIC_VALUE);
\PHPStan\dumpType(FinalRelative::STATIC_HOLDER);
function ordinary(A $open, FinalA $final): void {
    \PHPStan\dumpType($open::V);
    \PHPStan\dumpType($open::TYPED);
    \PHPStan\dumpType($open::DOC);
    \PHPStan\dumpType($final::TYPED);
}
