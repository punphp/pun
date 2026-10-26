<?php
namespace InitializerSafety;
function outside(): void {
    self::VALUE;
    parent::VALUE;
}
class NoParent {
    const VALUE = parent::VALUE;
}
class DirectSelf {
    const VALUE = self::VALUE;
}
enum Item { case One; }
class Cached {
    const ITEM = Item::One;
    function inspect(): void {
        self::ITEM->name;
        self::ITEM->name;
        static::ITEM->name;
        static::ITEM->name;
    }
}
class Numeric { const ONE = 1; }
enum Code: int { case One = Numeric::ONE; }
\PHPStan\dumpType(Code::One->value);
