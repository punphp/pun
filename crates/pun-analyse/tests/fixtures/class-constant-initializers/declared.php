<?php
namespace InitializerNextDeclared;
class Declared {
    const RAW = true ? 1 : 4;
    const int TYPED = true ? 1 : 4;
    /** @var int */ const DOC_INT = true ? 1 : 4;
    /** @var 1|4 */ const DOC_UNION = true ? 1 : 4;
    /** @var string */ const DOC_MISMATCH = true ? 1 : 4;
    /** @var 1|4 */ const int BOTH = true ? 1 : 4;
    final const FINAL_RAW = true ? 1 : 4;
    final const int FINAL_TYPED = true ? 1 : 4;
    const COPY_TYPED = self::TYPED;
    const COPY_DOC = self::DOC_INT;
    const COPY_BOTH = self::BOTH;
    function methods(): void {
        \PHPStan\dumpType(self::RAW);
        \PHPStan\dumpType(self::TYPED);
        \PHPStan\dumpType(self::DOC_INT);
        \PHPStan\dumpType(self::DOC_UNION);
        \PHPStan\dumpType(self::DOC_MISMATCH);
        \PHPStan\dumpType(self::BOTH);
        \PHPStan\dumpType(static::RAW);
        \PHPStan\dumpType(static::TYPED);
        \PHPStan\dumpType(static::DOC_INT);
        \PHPStan\dumpType(static::DOC_UNION);
        \PHPStan\dumpType(static::BOTH);
        \PHPStan\dumpType(static::FINAL_RAW);
        \PHPStan\dumpType(static::FINAL_TYPED);
        \PHPStan\dumpType($this::RAW);
        \PHPStan\dumpType($this::TYPED);
        \PHPStan\dumpType($this::DOC_INT);
    }
}
final class FinalChild extends Declared {
    function methods2(): void {
        \PHPStan\dumpType(static::RAW);
        \PHPStan\dumpType(static::TYPED);
        \PHPStan\dumpType(static::DOC_INT);
        \PHPStan\dumpType(static::FINAL_RAW);
    }
}
\PHPStan\dumpType(Declared::RAW);
\PHPStan\dumpType(Declared::TYPED);
\PHPStan\dumpType(Declared::DOC_INT);
\PHPStan\dumpType(Declared::DOC_UNION);
\PHPStan\dumpType(Declared::DOC_MISMATCH);
\PHPStan\dumpType(Declared::BOTH);
\PHPStan\dumpType(Declared::COPY_TYPED);
\PHPStan\dumpType(Declared::COPY_DOC);
\PHPStan\dumpType(Declared::COPY_BOTH);
#[\Attribute(Declared::TYPED)] class TypedFlag {}
#[\Attribute(Declared::DOC_INT)] class DocFlag {}
#[\Attribute(Declared::BOTH)] class BothFlag {}
#[TypedFlag, DocFlag, BothFlag] class Consumer {}
