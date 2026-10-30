<?php

namespace Pun\Oracle\SafeStrict;

function f(string $s): void
{
    preg_match("/a/", $s, $m);
}
