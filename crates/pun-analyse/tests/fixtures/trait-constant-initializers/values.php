<?php
namespace TraitInitializers;

trait Inputs {
    public const SELF_VALUE = self::SEED;
    public const PARENT_VALUE = parent::SEED;
    public const LONG = true ? self::SEED : parent::SEED;
    public const SHORT = self::ZERO ?: parent::SEED;
    public const COALESCE = self::ZERO ?? parent::SEED;
    public const SHAPE = ['self' => self::SEED, 'parent' => parent::SEED];
    public const FORWARD = self::LATER ?: SCANNED_ZERO ?: Scanned::FALLBACK;
    public const LATER = self::ZERO;
}
class FirstParent { public const SEED = 4; }
class SecondParent { public const SEED = 8; }
class First extends FirstParent { use Inputs; public const SEED = 1; public const ZERO = 0; }
class Second extends SecondParent { use Inputs; public const SEED = 2; public const ZERO = 0; }
class Inherited extends First { public const SEED = 16; }
class Reused extends First { use Inputs; public const SEED = 32; }
trait Nested { use Inputs; }
class Wrapped extends SecondParent { use Nested; public const SEED = 64; public const ZERO = 0; }

\PHPStan\dumpType(First::SELF_VALUE);
\PHPStan\dumpType(Second::SELF_VALUE);
\PHPStan\dumpType(First::PARENT_VALUE);
\PHPStan\dumpType(Second::PARENT_VALUE);
\PHPStan\dumpType(First::LONG);
\PHPStan\dumpType(Second::LONG);
\PHPStan\dumpType(First::SHORT);
\PHPStan\dumpType(First::COALESCE);
\PHPStan\dumpType(First::SHAPE);
\PHPStan\dumpType(Second::SHAPE);
\PHPStan\dumpType(First::FORWARD);
\PHPStan\dumpType(Second::FORWARD);
\PHPStan\dumpType(Inherited::SELF_VALUE);
\PHPStan\dumpType(Inherited::PARENT_VALUE);
\PHPStan\dumpType(Inherited::LONG);
\PHPStan\dumpType(Reused::SELF_VALUE);
\PHPStan\dumpType(Reused::PARENT_VALUE);
\PHPStan\dumpType(Reused::LONG);
\PHPStan\dumpType(Wrapped::SELF_VALUE);
\PHPStan\dumpType(Wrapped::PARENT_VALUE);

trait Declared {
    /** @var positive-int */ public const DOCUMENTED = true ? 1 : 4;
    public const int NATIVE = true ? 1 : 4;
    final public const int FINAL_VALUE = true ? 1 : 4;
}
class DeclaredConsumer {
    use Declared;
    public static function values(): void {
        \PHPStan\dumpType(self::DOCUMENTED);
        \PHPStan\dumpType(static::DOCUMENTED);
        \PHPStan\dumpType(self::NATIVE);
        \PHPStan\dumpType(static::NATIVE);
        \PHPStan\dumpType(static::FINAL_VALUE);
    }
}
\PHPStan\dumpType(DeclaredConsumer::DOCUMENTED);
\PHPStan\dumpType(DeclaredConsumer::NATIVE);
