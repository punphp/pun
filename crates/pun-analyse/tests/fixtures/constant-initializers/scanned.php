<?php

namespace ScannedInitializers {
    use const ForwardInitializers\KEY as SOURCE;
    const CHAIN = REMAINDER + SOURCE;
    const REMAINDER = 5.9 % 2.0;
    const TEXT = 'scanned-' . CHAIN;
    const NAMESPACE_NAME = __NAMESPACE__;
    define('ScannedInitializers\\DEFINED', 5.9 % 2.0);

    // A scanned declaration is available, but its unrelated body must never be analyzed.
    function notAnalysed(): void
    {
        \PHPStan\dumpType($notDefined);
    }
}

namespace ScannedInitializers {
    const BRACED = CHAIN + 1;
}
