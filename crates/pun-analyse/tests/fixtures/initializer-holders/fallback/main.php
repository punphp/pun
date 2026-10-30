<?php
namespace HolderFallback;
final class A { public const V = 1; }
final class B { public const V = 4; }
final class C { public const V = 7; }
class Values {
    public const AB_HOLDER = true ? new A : (false ? new B : C::class);
    public const BA_HOLDER = true ? new B : (false ? new A : C::class);
    public const AB_VALUE = (true ? new A : (false ? new B : C::class))::V;
    public const BA_VALUE = (true ? new B : (false ? new A : C::class))::V;
    public const STRING_OBJECT = (true ? B::class : new A)::V;
    public const OBJECT_STRING = (true ? new A : B::class)::V;
    public const STRING_UNKNOWN = (true ? new A : 'NoSuchFallbackHolder')::V;
    public const MAGIC_UNKNOWN = (true ? new A : NoSuchFallbackHolder::class)::V;
    public const BARE_OBJECT = (new (A::class))::V;
    public const CONDITIONAL_CLASSES = (true ? 'HolderFallback\\A' : 'HolderFallback\\B')::V;
}
\PHPStan\dumpType(Values::AB_HOLDER);
\PHPStan\dumpType(Values::BA_HOLDER);
\PHPStan\dumpType(Values::AB_VALUE);
\PHPStan\dumpType(Values::BA_VALUE);
\PHPStan\dumpType(Values::STRING_OBJECT);
\PHPStan\dumpType(Values::OBJECT_STRING);
\PHPStan\dumpType(Values::STRING_UNKNOWN);
\PHPStan\dumpType(Values::MAGIC_UNKNOWN);
\PHPStan\dumpType(Values::BARE_OBJECT);
\PHPStan\dumpType(Values::CONDITIONAL_CLASSES);
