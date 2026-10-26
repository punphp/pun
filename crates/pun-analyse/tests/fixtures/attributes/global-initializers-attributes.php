<?php

namespace GlobalInitializerScannedAttributes;

use const GlobalInitializerScanned\ZERO as SOURCE;
use const GlobalInitializerScanned\SPLIT;

#[\Attribute(SOURCE ?: 1)] class ScannedValid {}
#[\Attribute(SPLIT)] class ScannedInvalid {}

function notAnalysed(): void
{
    \PHPStan\dumpType($alsoNotDefined);
}
