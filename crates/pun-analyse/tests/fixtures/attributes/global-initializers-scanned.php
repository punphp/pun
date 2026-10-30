<?php

namespace GlobalInitializerScanned;

const ZERO = 0;
const NIL = null;
const SPLIT = true ? 1 : 4;

function notAnalysed(): void
{
    \PHPStan\dumpType($notDefined);
}
