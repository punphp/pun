<?php

namespace CompoundDuplicateKeys;

use const ForeignDuplicateKeys\SUM as FOREIGN_SUM;

const SUM = 1.2 + 1.7;
const DIFFERENCE = 5.8 - 3.2;
const PRODUCT = 1.4 * 2.0;
const QUOTIENT = 5.9 / 2.0;
const POWER = 1.7 ** 2;
const NEGATED = -(1.2 + 1.7);
const CHAIN = SUM + 1.0;
const CONDITIONAL = true ? SUM : 8.0;
const COALESCED = null ?? SUM;
const STRING_KEY = '1' . '2';
const SIZE = PHP_INT_SIZE + 1;
const BOOLEAN_KEY = true;
const NULL_KEY = null;
const ARRAY_KEY = [1];
const AFTER_CONTROLS = PRODUCT + 4.0;
const ITEMS = [1.2 + 1.7];
const OFFSET = ITEMS[0];

class ParentKeys
{
    public const SUM = 1.2 + 1.7;
    public const CHAIN = self::SUM + 1.0;
    public const WHICH = 'parent';
    public const LEXICAL = self::WHICH . '-key';
    final public const FINAL_SUM = 1.2 + 1.7;
    public const ITEMS = [self::SUM];
}

final class ChildKeys extends ParentKeys
{
    public const WHICH = 'child';
    public const PARENT_SUM = parent::SUM + 2.0;
    public const OFFSET = parent::ITEMS[0] + 1.0;
}

function keys(): void
{
    $a = [SUM => 0, 2 => 1];
    $b = [DIFFERENCE => 0, 2 => 1];
    $c = [PRODUCT => 0, 2 => 1];
    $d = [QUOTIENT => 0, 2 => 1];
    $e = [POWER => 0, 2 => 1];
    $f = [NEGATED => 0, -2 => 1];
    $g = [CHAIN => 0, 3 => 1];
    $h = [CONDITIONAL => 0, 2 => 1];
    $i = [COALESCED => 0, 2 => 1];
    $j = [STRING_KEY => 0, 12 => 1];
    $k = [ParentKeys::SUM => 0, 2 => 1];
    $l = [ChildKeys::CHAIN => 0, 3 => 1];
    $m = [ChildKeys::LEXICAL => 0, 'parent-key' => 1, 'child-key' => 2];
    $n = [ChildKeys::PARENT_SUM => 0, 4 => 1];
    $o = [FOREIGN_SUM => 0, 2 => 1];
    $p = [5 => 0, 9 => 1, SIZE => 2];
    $q = [SUM => 0, 3 => 1];
    $r = [BOOLEAN_KEY => 0, 1 => 1];
    $s = [NULL_KEY => 0, '' => 1];
    $t = [ARRAY_KEY => 0, 0 => 1];
    $u = [AFTER_CONTROLS => 0, 6 => 1];
    $v = [OFFSET => 0, 2 => 1];
    $w = [ChildKeys::OFFSET => 0, 3 => 1];
    $x = [ChildKeys::OFFSET => 0, 4 => 1];
}

/** @param class-string<ParentKeys> $class */
function late(string $class): void
{
    $a = [$class::SUM => 0, 2 => 1];
    $b = [$class::FINAL_SUM => 0, 2 => 1];
}
