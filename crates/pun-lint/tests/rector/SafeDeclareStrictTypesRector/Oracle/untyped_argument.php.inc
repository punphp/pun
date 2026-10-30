<?php

namespace Pun\Oracle\SafeStrict;

function g(int $i): void
{
}

function f($x): void
{
    g($x);
}
