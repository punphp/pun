<?php

namespace Pun\Oracle\SafeStrict;

function f(object $o, string $p): void
{
    $o->$p = 1;
}
