<?php
namespace TraitInitializers;
class ScanParent { public const FLAGS = 1; }
class ScanFirst extends ScanParent { use ScannedInputs; public const ZERO = 0; }
class ScanSecond extends ScanParent { use ScannedInputs; public const ZERO = null; }
\PHPStan\dumpType(ScanFirst::FORWARD);
\PHPStan\dumpType(ScanSecond::FORWARD);
\PHPStan\dumpType(Second::LONG);
\PHPStan\dumpType(First::LONG);
#[TraitText(self::LOCAL)]
trait ReboundAttribute {}
class TextAttributeConsumer { use ReboundAttribute; public const LOCAL = 'valid'; }
class NumberAttributeConsumer { use ReboundAttribute; public const LOCAL = 3; }
class InheritedAttributeConsumer extends TextAttributeConsumer { public const LOCAL = 4; }
class ReusedAttributeConsumer extends TextAttributeConsumer { use ReboundAttribute; public const LOCAL = 5; }

trait ShadowedValue { public const VALUE = self::SEED; }
class ShadowedA { use ShadowedValue; public const SEED = 1; public const VALUE = 1; }
class ShadowedB { use ShadowedValue; public const SEED = 2; public const VALUE = 1; }
class ShadowedC { use ShadowedValue; public const SEED = 1; public const VALUE = 2; }

trait DocumentedMismatch { /** @var string */ public const VALUE = true ? 1 : 4; }
class MismatchConsumer { use DocumentedMismatch; }
\PHPStan\dumpType(MismatchConsumer::VALUE);
trait Untyped { public const VALUE = true ? 1 : 4; }
class UntypedConsumer {
    use Untyped;
    public static function values(): void {
        \PHPStan\dumpType(self::VALUE);
        \PHPStan\dumpType(static::VALUE);
    }
}
final class FinalUntypedConsumer {
    use Untyped;
    public static function values(): void { \PHPStan\dumpType(static::VALUE); }
}
