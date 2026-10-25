<?php
namespace NativeCasts;

/** @param positive-int $integer
 * @param non-empty-string $text
 * @param true $flag
 */
function parameters(int $integer, float $decimal, string $text, bool $flag, mixed $unknown): void
{
    (int) $integer; // native
    (integer) $decimal; // native
    (int) $text; // native
    (int) $flag; // native
    (int) $unknown; // native
    (float) $integer; // native
    (double) $decimal; // native
    (float) $text; // native
    (float) $flag; // native
    (float) $unknown; // native
    (bool) $integer; // native
    (boolean) $decimal; // native
    (bool) $text; // native
    (bool) $flag; // native
    (bool) $unknown; // native
    (string) $text; // native
    (binary) $decimal; // native
    (string) $flag; // native
    (string) $unknown; // native
    $copy = (int) '12.9tail';
    $copy; // native
    $copy = (string) false;
    $copy; // native
    $spelling = (string) $decimal;
    $spelling; // native
    (string) $spelling; // native
    (bool) $spelling; // native
    (int) ($integer % 2); // native
    (float) ($integer % 2); // native
    (bool) ($integer % 2); // native
    (string) ('prefix' . $text); // native
    (bool) ('prefix' . $text); // native
}

function constants(): void
{
    (int) null; // native
    (float) null; // native
    (bool) null; // native
    (string) null; // native
    (int) true; // native
    (int) false; // native
    (float) true; // native
    (float) false; // native
    (bool) true; // native
    (bool) false; // native
    (string) true; // native
    (string) false; // native
    (int) 42; // native
    (float) 42; // native
    (bool) 0; // native
    (bool) -1; // native
    (string) 0; // native
    (string) -42; // native
    (string) 9223372036854775807; // native
    (string) (-9223372036854775807 - 1); // native
    (float) (-9223372036854775807 - 1); // native
    (int) 12.9; // native
    (int) -12.9; // native
    (float) 12.9; // native
    (bool) 0.0; // native
    (bool) -0.0; // native
    (bool) -0.5; // native
    (int) 9223372036854775808.0; // native
    (int) -9223372036854775808.0; // native
    (int) 18446744073709551616.0; // native
    (int) -18446744073709551616.0; // native
    (int) 1e20; // native
    (int) -1e20; // native
    (int) 1e308; // native
    (int) -1e308; // native
    (int) -9223372036854777856.0; // native
    (string) 0.0; // native
    (string) -0.0; // native
    (string) 1.0; // native
    (string) -12.5; // native
    (string) 1.2345678901234567; // native
    (string) 1.23456789012345; // native
    (string) 1.23456789012355; // native
    (string) -0.125; // native
    (string) 1e-4; // native
    (string) 1e-5; // native
    (string) 1e13; // native
    (string) 1e14; // native
    (string) 99999999999999.9; // native
    (string) -1e20; // native
    (string) 9223372036854775808.0; // native
    (bool) ''; // native
    (bool) '0'; // native
    (bool) '00'; // native
    (bool) '0.0'; // native
    (bool) '-0'; // native
    (bool) 'false'; // native
    (bool) "\0"; // native
    (string) '0012'; // native
    (int) ''; // native
    (int) " \t\n\r\v\f"; // native
    (int) 'word'; // native
    (int) '0x10'; // native
    (int) '0012'; // native
    (int) '  +12.9e2tail'; // native
    (int) "\t\n\r\v\f -12.9tail"; // native
    (int) "\xc2\xa012"; // native
    (int) "12\xff"; // native
    (int) '.9'; // native
    (int) '-.9'; // native
    (int) '+'; // native
    (int) '.'; // native
    (int) '1e+'; // native
    (int) '2E-2tail'; // native
    (int) '9223372036854775807'; // native
    (int) '-9223372036854775808'; // native
    (int) '9223372036854775808'; // native
    (int) '-9223372036854775809'; // native
    (int) '1e999'; // native
    (int) '-1e999'; // native
    (int) '1e-999'; // native
    (float) ''; // native
    (float) " \t\n\r\v\f"; // native
    (float) 'word'; // native
    (float) 'INF'; // native
    (float) 'NAN'; // native
    (float) '0x10'; // native
    (float) '  +12.9e2tail'; // native
    (float) "\v\f-12.5e-1rest"; // native
    (float) "\xc2\xa012"; // native
    (float) "12\xff"; // native
    (float) '.5'; // native
    (float) '-.5'; // native
    (float) '+'; // native
    (float) '.'; // native
    (float) '1e+'; // native
    (float) '2E-2tail'; // native
    (float) '1e-999'; // native
}

function nested(): void
{
    (int) (float) ' 12.5suffix'; // native
    (string) (int) '0012'; // native
    (bool) (string) 0; // native
    (float) (bool) ''; // native
    (int) (true xor false); // native
    (int) (7 / 2); // native
}
