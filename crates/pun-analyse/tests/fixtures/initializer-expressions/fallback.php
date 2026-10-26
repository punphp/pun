<?php
namespace InitializerExpressionFallback;
function number(): int { return 1; }
trait CallInput { public const VALUE = number(); }
class LiteralMismatch { use CallInput; public const VALUE = 1; }
trait LiteralInput { public const VALUE = 1; }
class CallMismatch { use LiteralInput; public const VALUE = number(); }
class SameCall { use CallInput; public const VALUE = number(); }
class Values {
    public const CALL = number();
    public const ARRAY_VALUE = ['value' => number()];
    public const TERNARY = false ? number() : 1;
    public const SHORTHAND = number() ?: 1;
    public const COALESCE = number() ?? 1;
    public const NESTED = ['value' => number() ?? 1];
    public const ARITHMETIC = 1 + 2 * 3;
    /** @var int */ public const DOC = number();
    public const int NATIVE = number();
}
\PHPStan\dumpType(Values::CALL);
\PHPStan\dumpType(Values::ARRAY_VALUE);
\PHPStan\dumpType(Values::TERNARY);
\PHPStan\dumpType(Values::SHORTHAND);
\PHPStan\dumpType(Values::COALESCE);
\PHPStan\dumpType(Values::NESTED);
\PHPStan\dumpType(Values::ARITHMETIC);
\PHPStan\dumpType(Values::DOC);
\PHPStan\dumpType(Values::NATIVE);
\PHPStan\dumpType(GLOBAL_VALUE);
const GLOBAL_VALUE = number();
\PHPStan\dumpType(GLOBAL_VALUE);
\PHPStan\dumpType(SCANNED_CALL);
\PHPStan\dumpType(Scanned::CALL);
\PHPStan\dumpType(Scanned::ARRAY_VALUE);
class StaticCalls {
    public static function number(): int { return 1; }
    public const CALL = self::number();
}
\PHPStan\dumpType(StaticCalls::CALL);
\PHPStan\dumpType(StaticCalls::number());
#[\Attribute(Values::CALL)] class InvalidAttribute {}
#[InvalidAttribute] class InvalidAttributeUse {}
