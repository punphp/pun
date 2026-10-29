<?php

namespace Pun\Oracle\SafeStrict;

function f(): int
{
    return g();
}

function g()
{
    return 1;
}
