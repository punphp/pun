<?php
namespace NativeOperators;

/** @param positive-int $integer
 * @param non-empty-string $text
 * @param true $flag
 */
function parameters(int $integer, float $decimal, bool $flag, string $text): void
{
    +$integer; // native
    -$integer; // native
    +$decimal; // native
    -$decimal; // native
    !$integer; // native
    !$flag; // native
    !$text; // native
    $integer + 2; // native
    2 + $integer; // native
    $integer + $integer; // native
    $integer - 2; // native
    $integer * 2; // native
    $integer / 2; // native
    $integer % 2; // native
    $decimal + 1; // native
    $integer + $decimal; // native
    $integer * 0; // native
    0 * $decimal; // native
    $decimal * 0; // native
    0.0 * $decimal; // native
    $integer / 1; // native
    $integer / -1; // native
    $integer / $integer; // native
    0 / $integer; // native
    $integer % -1; // native
    $integer - $integer; // native
    $decimal / 2; // native
    2 / $decimal; // native
    $integer === 2; // native
    $integer === $integer; // native
    $integer !== $text; // native
    $integer < 2; // native
    $integer <=> 2; // native
    $integer <=> $integer; // native
    $integer == $integer; // native
    $decimal === ($decimal); // native
    $flag === true; // native
    $text === 'yes'; // native
    $text === $text; // native
    $text . $text; // native
    $text . ''; // native
    '' . $text; // native
    'prefix' . $text; // native
    '0' . $text; // native
    $text . '0'; // native
    '0' . $text . '0'; // native
    $flag xor $flag; // native
    $integer xor $text; // native
    $copy = $integer + 1;
    $copy; // native
    $copy = 3 * 4;
    $copy; // native
}

function constants(): void
{
    +7; // native
    -7; // native
    -(-7); // native
    !0; // native
    !1; // native
    !false; // native
    !null; // native
    !''; // native
    !'0'; // native
    !'yes'; // native
    7 + 3; // native
    7 - 10; // native
    7 * 3; // native
    6 / 3; // native
    7 / 2; // native
    -7 % 3; // native
    7 % -3; // native
    1.5 + 2; // native
    2 - 1.5; // native
    2.5 * 2; // native
    2.5 / 2; // native
    -1.5; // native
    +1.5; // native
    1.5 === 1.5; // native
    1.5 !== 2.5; // native
    9223372036854775807 + 1; // native
    9223372036854775807 * 2; // native
    (-9223372036854775807 - 1) - 1; // native
    -(-9223372036854775807 - 1); // native
    7 === 7; // native
    7 !== 7; // native
    7 === 7.0; // native
    7 == 7.0; // native
    9007199254740992 == 9007199254740993; // native
    9007199254740992 < 9007199254740993; // native
    7 != 8; // native
    7 <> 7; // native
    7 < 8; // native
    7 <= 7; // native
    7 > 8; // native
    7 >= 7; // native
    7 <=> 8; // native
    7 <=> 7; // native
    8 <=> 7; // native
    'same' === 'same'; // native
    'a' !== 'b'; // native
    true === false; // native
    null === null; // native
    true xor false; // native
    true xor true; // native
    'a' . 'b'; // native
    'a' . 'b' . 'c'; // native
    ('a' . 'b') . 'c'; // native
}

function minimum(): void
{
    $minimum = -9223372036854775807 - 1;
    /** @var int $minimum */
    $other = 0;
    $minimum / -1; // native
    $minimum % -1; // native
}

function defaults(int $integer = 1 + 2, float $decimal = 1.5 * 2): void
{
    $integer; // native
    $decimal; // native
}
