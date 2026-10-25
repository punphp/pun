<?php

namespace ScalarDuplicateKeys;

use const SEEK_CUR as NEXT;
use SplFileObject as File;

const FRACTION = -1.9;
const FRACTION_ALIAS = FRACTION;
const LOCAL = 'local';
const SEEK_SET = 9;
const BUILTIN_ALIAS = \SEEK_CUR;
const UNION_ALIAS = \PHP_INT_MAX;

function floats(float $unknown): void
{
    $a = [-1.9 => 0, -1 => 1, +1.9 => 2, 1 => 3];
    $b = [-0.0 => 0, +0.0 => 1, 0 => 2];
    $c = [9.223372036854776E18 => 0, -9223372036854775808 => 1];
    $d = [-9.223372036854778E18 => 0, 9223372036854773760 => 1];
    $e = [1E20 => 0, 7766279631452241920 => 1];
    $f = [-1E20 => 0, -7766279631452241920 => 1];
    $g = [1E300 => 0, 0 => 1, -1E300 => 2];
    $h = [INF => 0, -INF => 1, NAN => 2, 0 => 3];
    $i = [$unknown => 0, 0 => 1, 'unknown', 1 => 2];
    $large = 9.223372036854776E18;
    $j = [$large => 0, -9223372036854775808 => 1];
    $finite = -1E20;
    $k = [$finite => 0, -7766279631452241920 => 1];
    $negative = -INF;
    $l = [$negative => 0, 0 => 1];
    $nan = NAN;
    $m = [$nan => 0, 0 => 1];
    $n = [FRACTION => 0, FRACTION_ALIAS => 1, -1 => 2];
    $o = [18446744073709549568.0 => 0, -2048 => 1];
    $p = [18446744073709551615 => 0, 0 => 1];
    $q = [-2147483647 => 0, -9223372036854775807 => 1, -PHP_INT_MAX => 2];
    $r = [-9223372036854775808 => 0, 2147483648 => 1, -PHP_INT_MIN => 2];
    $s = [+INF => 0, 0 => 1];
    $t = [-$unknown => 0, 0 => 1];
    $u = [!true => 0, 0 => 1];
    $v = [true => 0, 1 => 1, false => 2, 0 => 3, null => 4, '' => 5];
    $w = [1 + 1 => 0, 2 => 1];
    $x = [+'2.9' => 0, 2 => 1];
    $y = [-' 2 ' => 0, -2 => 1, +true => 2, 1 => 3, -false => 4, +null => 5, 0 => 6];
    $z = [-'not numeric' => 0, 0 => 1];
}

function constants(): void
{
    $a = [\SEEK_SET => 0, 0 => 1];
    $b = [NEXT => 0, 1 => 1];
    $c = [SEEK_SET => 0, 9 => 1];
    $d = [LOCAL => 0, \ScalarDuplicateKeys\LOCAL => 1];
    $e = [2147483647 => 0, 9223372036854775807 => 1, PHP_INT_MAX => 2];
    $f = [PHP_INT_MAX => 0, 9223372036854775807 => 1];
    $g = [0 => 0, 1 => 1, PHP_DEBUG => 2];
    $h = [PHP_OS => 0, '' => 1];
    $i = [PHP_VERSION_ID => 0, 80400 => 1];
    $j = [PHP_MAJOR_VERSION => 0, 8 => 1];
    $k = [PHP_MINOR_VERSION => 0, 4 => 1];
    $l = [PHP_RELEASE_VERSION => 0, 0 => 1];
    $m = [File::READ_CSV => 0, 8 => 1];
    $n = [\Attribute::TARGET_CLASS => 0, 1 => 1];
    $o = [\DateTime::ATOM => 0, 'Y-m-d\TH:i:sP' => 1];
    $p = [BUILTIN_ALIAS => 0, 1 => 1];
    $q = [2147483647 => 0, 9223372036854775807 => 1, UNION_ALIAS => 2];
}

class ParentKeys
{
    public const INHERITED = 'inherited';
    public const FLOAT_KEY = -1.9;
    public const OPEN = 'open';
    final public const FIXED = 'fixed';
    public const int NATIVE = 7;
    /** @var 'doc'|'other' */
    public const DOC = 'doc';
    /** @final */
    public const DOC_FINAL = 'doc-final';
    public const ALIAS_SELF = self::OPEN;
    public const REFERENCED_FLOAT = self::FLOAT_KEY;
    final public const FINAL_FLOAT = self::FLOAT_KEY;
    final public const int FINAL_NATIVE = 31;
    /** @var 'final-doc'|'other' */
    final public const FINAL_DOC = 'final-doc';

    public function keys(): void
    {
        $a = [static::OPEN => 0, 'open' => 1];
        $b = [static::FIXED => 0, 'fixed' => 1];
        $c = [static::NATIVE => 0, 7 => 1];
        $d = ['doc' => 0, 'other' => 1, static::DOC => 2];
        $e = [self::NATIVE => 0, 7 => 1];
        $f = [self::DOC => 0, 'doc' => 1];
        $g = [static::DOC_FINAL => 0, 'doc-final' => 1];
        $h = [static::class => 0, ParentKeys::class => 1];
        $i = [static::FINAL_NATIVE => 0, 31 => 1];
        $j = [static::FINAL_DOC => 0, 'final-doc' => 1];
    }
}

class ChildKeys extends ParentKeys
{
    public const OPEN = 'child';
    public const FLOAT_KEY = 2.9;

    public function inherited(): void
    {
        $a = [self::INHERITED => 0, 'inherited' => 1];
        $b = [parent::FLOAT_KEY => 0, -1 => 1];
        $c = [self::OPEN => 0, 'child' => 1];
        $d = [parent::OPEN => 0, 'open' => 1];
        $e = [static::FIXED => 0, 'fixed' => 1];
        $f = [self::ALIAS_SELF => 0, 'open' => 1];
        $g = [self::REFERENCED_FLOAT => 0, -1 => 1];
        $h = [static::FINAL_FLOAT => 0, -1 => 1];
    }
}

final class FinalKeys extends ParentKeys
{
    public function resolved(): void
    {
        $a = [static::OPEN => 0, 'open' => 1];
        $b = [static::NATIVE => 0, 7 => 1];
        $c = [static::DOC => 0, 'doc' => 1];
        $d = [static::class => 0, FinalKeys::class => 1];
    }
}

/** @final */
class DocFinalKeys extends ParentKeys
{
    public function resolved(): void
    {
        $a = [static::OPEN => 0, 'open' => 1];
    }
}

interface InterfaceKeys
{
    public const KEY = 23;
}

class InterfaceChild implements InterfaceKeys
{
    public function keys(): void
    {
        $a = [self::KEY => 0, 23 => 1];
        $b = [static::KEY => 0, 23 => 1];
    }
}

class BuiltinChild extends \SplFileObject
{
    public function keys(): void
    {
        $a = [self::READ_CSV => 0, 8 => 1];
        $b = [parent::READ_CSV => 0, 8 => 1];
        $c = [static::READ_CSV => 0, 8 => 1];
    }
}

/** @param class-string<ParentKeys> $class */
function receivers(ParentKeys $object, string $class, string $name): void
{
    $a = [$object::OPEN => 0, 'open' => 1];
    $b = [$object::FIXED => 0, 'fixed' => 1];
    $c = [$class::FIXED => 0, 'fixed' => 1];
    $exact = ParentKeys::class;
    $d = [$exact::OPEN => 0, 'open' => 1];
    $e = [ParentKeys::{$name} => 0, 'open' => 1];
    $f = ['ScalarDuplicateKeys\ParentKeys'::OPEN => 0, 'open' => 1];
    $qualified = '\ScalarDuplicateKeys\ParentKeys';
    $g = [$qualified::OPEN => 0, 'open' => 1];
    // ClassConstFetchHandler returns mixed even when the dynamic selector is a constant string.
    $knownName = 'FIXED';
    $h = [ParentKeys::{$knownName} => 0, 'fixed' => 1];
}

class AlternateKeys
{
    final public const FIXED = 'alternate';
    final public const OPEN = 'alternate-open';
}

/**
 * @template T of ParentKeys
 * @param T $template
 * @param ParentKeys&InterfaceKeys $intersection
 * @param class-string $unknown
 */
function compoundReceivers(ParentKeys|AlternateKeys $union, ParentKeys $template, ParentKeys $intersection, string $unknown): void
{
    $a = ['fixed' => 0, 'alternate' => 1, $union::FIXED => 2];
    $b = [$template::FIXED => 0, 'fixed' => 1];
    $c = [$intersection::FIXED => 0, 'fixed' => 1];
    $d = [$unknown::FIXED => 0, 'fixed' => 1];
    // The open arm must not disappear from the union merely because it has no precise value.
    $e = [$union::OPEN => 0, 'open' => 1, 'alternate-open' => 2];
}

function replaceFloat(float &$value, float $replacement): void
{
    $value = $replacement;
}

function copiedNonFinite(float $unknown, bool $condition): void
{
    $first = NAN;
    $second = $first;
    $a = [$second => 0, 0 => 1];
    $parenthesized = [(($second)) => 0, 0 => 1];
    $first = 1.5;
    $b = [$first => 0, 0 => 1, 1 => 2];
    $escaped = NAN;
    replaceFloat($escaped, $unknown);
    $c = [$escaped => 0, 0 => 1];
    $joined = NAN;
    if ($condition) {
        $joined = $unknown;
    }
    $d = [$joined => 0, 0 => 1];
    $fromUnknown = $unknown;
    $e = [$fromUnknown => 0, 0 => 1];
}

function copiedBuiltinConstants(): void
{
    $debug = PHP_DEBUG;
    $a = [$debug => 0, 0 => 1];
    $b = [0 => 0, 1 => 1, $debug => 2];
    assert($debug === 0);
    $c = [$debug => 0, 0 => 1, 1 => 2];
    $debug = PHP_DEBUG;
    assert($debug === 1);
    $d = [$debug => 0, 0 => 1, 1 => 2];
}

namespace ShadowBuiltinKeys;

const PHP_DEBUG = 1.2 + 1.7;

function existingUnknownValueStopsBuiltinFallback(): void
{
    $a = [0 => 0, 1 => 1, PHP_DEBUG => 2];
}
