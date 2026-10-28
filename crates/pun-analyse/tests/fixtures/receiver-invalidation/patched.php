<?php

namespace ReceiverInvalidation;

use function PHPStan\dumpType;

function patched(PatchedBox $box): void
{
    if ($box->value !== null) {
        $box->unknown();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->impure();
        dumpType($box->value);
    }
}
