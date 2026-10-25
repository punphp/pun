<?php

use function PHPStan\dumpType;
use function PHPStan\dumpPhpDocType;

dumpType('kept'); // @phpstan-ignore phpstan.dumpType
dumpPhpDocType('kept'); // @phpstan-ignore phpstan.dumpPhpDocType
