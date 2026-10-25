<?php

namespace RandomVersions;

use function PHPStan\dumpType;

function endpoints(): void
{
    dumpType(rand());
    dumpType(mt_rand());
    dumpType(rand(13, -2));
    dumpType(random_int(-11, 3));
}

function invalidRandomInt(): string
{
    return random_int(13, -2);
}

function invalidMtRand(): string
{
    return mt_rand(13, -2);
}

function nestedInvalid(): string
{
    return random_int(9, random_int(1, 8));
}

function validControl(): string
{
    return random_int(-5, 17);
}
