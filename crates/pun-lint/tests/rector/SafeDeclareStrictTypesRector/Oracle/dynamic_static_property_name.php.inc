<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public static int $s = 0;
}

function f(string $p): void
{
    A::$$p = 1;
}
