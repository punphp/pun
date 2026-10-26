<?php
namespace HolderOrder;
class EmptyHolder {}
final class A { public const V = EmptyHolder::ABSENT; }
final class B { public const V = strlen('x'); }
class Values {
    public const AB_HOLDER = true ? A::class : B::class;
    public const BA_HOLDER = true ? B::class : A::class;
    public const AB = (true ? A::class : B::class)::V;
    public const BA = (true ? B::class : A::class)::V;
    public const ALIAS_AB = (self::AB_HOLDER)::V;
    public const ALIAS_BA = (self::BA_HOLDER)::V;
}
\PHPStan\dumpType(A::V);
\PHPStan\dumpType(B::V);
\PHPStan\dumpType(Values::AB_HOLDER);
\PHPStan\dumpType(Values::BA_HOLDER);
\PHPStan\dumpType(Values::AB);
\PHPStan\dumpType(Values::BA);
\PHPStan\dumpType(Values::ALIAS_AB);
\PHPStan\dumpType(Values::ALIAS_BA);
