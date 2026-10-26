<?php

namespace GlobalInitializerArrays;

function beforeDeclarations(): void
{
    \PHPStan\dumpType(LIST_VALUES);
    \PHPStan\dumpType(KEYED_VALUES);
    \PHPStan\dumpType(NULLABLE);
    \PHPStan\dumpType(MAYBE_ZERO);
}

const CANONICAL = true ? 1 : 4;
const NULLABLE = false ? 1 : null;
const MAYBE_ZERO = false ? 1 : 0;
const FLOAT_ZERO = 0.0;
const LIST_VALUES = [true ? 1 : 4, null ?? 1, true ? 0 : 1, 5.9 % 2.0];
const KEYED_VALUES = ['flag' => true ? 1 : 4, 'null' => null ?? 1];

class Keys
{
    public const UNION = CANONICAL;
    public const EXACT = 5.9 % 2.0;
    public const MAP = ['flag' => CANONICAL];
}

function afterDeclarations(): void
{
    \PHPStan\dumpType(LIST_VALUES);
    \PHPStan\dumpType(KEYED_VALUES);
    \PHPStan\dumpType(NULLABLE);
    \PHPStan\dumpType(MAYBE_ZERO);
    $union = [Keys::UNION => 'maybe one', 1 => 'not necessarily repeated'];
    $exact = [Keys::EXACT => 'one', 1 => 'repeated'];
    $unpack = [...Keys::MAP, 'flag' => 'repeated'];
}

#[\Attribute(NULLABLE ?? 1)] class RemoveNull {}
#[\Attribute(MAYBE_ZERO ?: 1)] class RemoveZero {}
#[\Attribute(FLOAT_ZERO ?: 1)] class RemoveFloatZero {}
#[\Attribute(MAYBE_ZERO ?? 1)] class RetainZero {}
#[\Attribute(NULLABLE ?: 1)] class RemoveNullFromTernary {}
#[\Attribute(((MAYBE_ZERO ?: 1) | 128))] class Repeated {}
#[\Attribute([true ? 1 : 1])] class ArrayIsNotFlags {}

#[RemoveNull, RemoveZero, RemoveFloatZero, RemoveNullFromTernary, Repeated, Repeated]
#[RetainZero, ArrayIsNotFlags]
class Consumer {}
