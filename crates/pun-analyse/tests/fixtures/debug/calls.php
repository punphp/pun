<?php

namespace PHPStan {
    dumpType(1);
    dumpPhpDocType(true);
}

namespace DebugCalls {
    use PHPStan as P;
    use function PHPStan\dumpType as dtype;
    use function PHPStan\dumpPhpDocType as ptype;

    dtype(2);
    \PHPStan\DuMpTyPe('case');
    ptype(null);
    \PHPStan\DuMpPhPdOcTyPe(false);
    P\dumpType(5);
    P\dumpPhpDocType(6);
    dtype(1, true);
    ptype([1], [2]);
    dtype(...[1, 2]);
    ptype(...['value']);
    dtype(...[1, 2], ...[3, 4]);
    dtype(value: 'named', extra: false);
    ptype(extra: true, value: 7);
    dtype(
        'one',
        'two'
    );
    ptype(
        3,
        4
    );
    dtype();
    ptype();
}
