<?php

namespace IssetNativeCertainty;

class NativeProperties
{
    public int $value = 1;
    public ?int $nullable = null;
}

/** @param NativeProperties $box */
function nullsafe(?NativeProperties $box): void
{
    var_dump(isset($box?->value), isset($box?->nullable));
    var_dump($box?->value ?? 0, $box?->nullable ?? 0);
}
