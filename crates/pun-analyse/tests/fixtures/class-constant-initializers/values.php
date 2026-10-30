<?php
namespace InitializerNext;

function beforeDeclarations(): void {
    \PHPStan\dumpType(GLOBAL_SPLIT);
    \PHPStan\dumpType(GLOBAL_COALESCE);
    \PHPStan\dumpType(GLOBAL_FALSEY);
    \PHPStan\dumpType(GLOBAL_SHORT);
    \PHPStan\dumpType(GLOBAL_SHAPE);
    \PHPStan\dumpType(Holder::SPLIT);
    \PHPStan\dumpType(Holder::COALESCE);
    \PHPStan\dumpType(Holder::FALSEY);
    \PHPStan\dumpType(Holder::SHORT);
    \PHPStan\dumpType(Holder::SHAPE);
}

const GLOBAL_SPLIT = true ? 1 : 4;
const GLOBAL_COALESCE = 0 ?? 1;
const GLOBAL_FALSEY = true ? 0 : 1;
const GLOBAL_SHORT = 0 ?: 1;
const GLOBAL_SHAPE = [GLOBAL_SPLIT, GLOBAL_COALESCE, GLOBAL_FALSEY];
const GLOBAL_FORWARD = GLOBAL_LATE ?: 1;
const GLOBAL_FROM_CLASS = Holder::ZERO ?: 1;
const GLOBAL_LATE = 0;

class Holder {
    const SPLIT = true ? 1 : 4;
    const COALESCE = 0 ?? 1;
    const FALSEY = true ? 0 : 1;
    const SHORT = 0 ?: 1;
    const SHAPE = [self::SPLIT, self::COALESCE, self::FALSEY];
    const FORWARD = self::LATE ?: 1;
    const FROM_SCANNED = Scanned::ZERO ?: 1;
    const FROM_SCANNED_NULL = Scanned::NIL ?? 1;
    const FROM_GLOBAL = GLOBAL_LATE ?: 1;
    const ZERO = 0;
    const LATE = 0;
    const NIL = null;
    const QUALIFIED = Scanned::SPLIT;
}

function afterDeclarations(): void {
    \PHPStan\dumpType(GLOBAL_SPLIT);
    \PHPStan\dumpType(GLOBAL_COALESCE);
    \PHPStan\dumpType(GLOBAL_FALSEY);
    \PHPStan\dumpType(GLOBAL_SHORT);
    \PHPStan\dumpType(GLOBAL_SHAPE);
    \PHPStan\dumpType(GLOBAL_FORWARD);
    \PHPStan\dumpType(GLOBAL_FROM_CLASS);
    \PHPStan\dumpType(Holder::SPLIT);
    \PHPStan\dumpType(Holder::COALESCE);
    \PHPStan\dumpType(Holder::FALSEY);
    \PHPStan\dumpType(Holder::SHORT);
    \PHPStan\dumpType(Holder::SHAPE);
    \PHPStan\dumpType(Holder::FORWARD);
    \PHPStan\dumpType(Holder::FROM_SCANNED);
    \PHPStan\dumpType(Holder::FROM_SCANNED_NULL);
    \PHPStan\dumpType(Holder::FROM_GLOBAL);
    \PHPStan\dumpType(Holder::QUALIFIED);
    \PHPStan\dumpType(Scanned::SHAPE);
}

class Grand {
    const ZERO = 0;
    const FIXED = true ? 1 : 4;
}
class ParentHolder extends Grand {
    const ZERO = 4;
    const VIA_PARENT = parent::ZERO ?: 1;
    const VIA_SELF = self::ZERO ?: 1;
    const PARENT_ARRAY = [parent::ZERO, self::ZERO, self::FIXED];
}
class ChildHolder extends ParentHolder {
    const ZERO = 16;
    function inspect(): void {
        \PHPStan\dumpType(self::VIA_PARENT);
        \PHPStan\dumpType(self::VIA_SELF);
        \PHPStan\dumpType(self::PARENT_ARRAY);
        \PHPStan\dumpType(parent::VIA_PARENT);
        \PHPStan\dumpType(parent::VIA_SELF);
        \PHPStan\dumpType(static::VIA_SELF);
        \PHPStan\dumpType(static::FIXED);
    }
}
\PHPStan\dumpType(ChildHolder::VIA_PARENT);
\PHPStan\dumpType(ChildHolder::VIA_SELF);
\PHPStan\dumpType(ChildHolder::PARENT_ARRAY);
\PHPStan\dumpType(ChildHolder::FIXED);

#[\Attribute(Holder::SPLIT)] class InvalidSplit {}
#[\Attribute(Holder::COALESCE)] class InvalidCoalesce {}
#[\Attribute(Holder::FALSEY)] class ValidFalsey {}
#[\Attribute(Holder::FORWARD)] class ValidForward {}
#[\Attribute(Holder::FROM_SCANNED_NULL)] class ValidScannedNull {}
#[\Attribute(ChildHolder::VIA_PARENT)] class ValidParent {}
#[\Attribute(ChildHolder::VIA_SELF)] class InvalidInheritedSelf {}
#[InvalidSplit, InvalidCoalesce, ValidFalsey, ValidForward, ValidScannedNull, ValidParent, InvalidInheritedSelf]
class Consumer {}
