<?php
namespace InitializerNeighbors;

trait NumericKeys {
    const FIRST = 1.5;
    const SECOND = self::FIRST + 0.5;
}
class UsingKeys { use NumericKeys; }
$keys = [UsingKeys::SECOND => 'first', 2 => 'second'];

class Ancestor {
    const RAW = true ? 1 : 4;
    const int TYPED = true ? 1 : 4;
}
/** @phpstan-require-extends Ancestor */
trait Required {
    function inspect(): void {
        \PHPStan\dumpType(self::RAW);
        \PHPStan\dumpType(static::TYPED);
    }
}
class UsingRequired extends Ancestor { use Required; }
