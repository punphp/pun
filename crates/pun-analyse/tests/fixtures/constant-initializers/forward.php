<?php

namespace ForwardInitializers {
    use const ScannedInitializers\REMAINDER as IMPORTED;
    use const ScannedInitializers\CHAIN;
    use const PHP_INT_SIZE as WORD_SIZE;

    function copies(): void
    {
        $value = KEY;
        \PHPStan\dumpType($value);
        $copy = $value;
        \PHPStan\dumpType($copy);
        $value = 7;
        \PHPStan\dumpType($value);
        \PHPStan\dumpType($copy);
        $array = [$copy => 0, 1 => 1];
        \PHPStan\dumpType(IMPORTED);
        $imported = CHAIN;
        \PHPStan\dumpType($imported);
        \PHPStan\dumpType(SECOND);
        \PHPStan\dumpType(DECLARED);
        $size = PLATFORM;
        \PHPStan\dumpType($size);
        assert($size === 5);
        \PHPStan\dumpType($size);
        $size = 12;
        \PHPStan\dumpType($size);
        \PHPStan\dumpType(LABEL);
        \PHPStan\dumpType(SHAPE);
        \PHPStan\dumpType(\ScannedInitializers\TEXT);
        \PHPStan\dumpType(\ScannedInitializers\DEFINED);
        \PHPStan\dumpType(\ScannedInitializers\NAMESPACE_NAME);
        \PHPStan\dumpType(\ScannedInitializers\BRACED);
    }

    const SECOND = KEY + IMPORTED;
    const KEY = 5.9 % 2.0;
    /** @var int */
    const DECLARED = 5.9 % 2.0;
    const PLATFORM = WORD_SIZE + 1;
    const LABEL = 'key-' . KEY;
    const SHAPE = [KEY, 'next' => SECOND];
}

namespace ShadowInitializers {
    const KEY = 9;
    function selected(): void
    {
        \PHPStan\dumpType(KEY);
        \PHPStan\dumpType(\ForwardInitializers\KEY);
        \PHPStan\dumpType(\ScannedInitializers\REMAINDER);
    }
}
