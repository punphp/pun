<?php
namespace RepeatBytes;
use function PHPStan\dumpType;

function bytes(bool $flag): void
{
    dumpType(str_repeat("\0\n\t\r\v\f\a\b\\\"'", 2));
    dumpType(str_repeat("\x01\x1f\x7f", 2));
    dumpType(str_repeat('é', 2));
    dumpType(str_repeat("\xff", 2) === "\xff\xff");
    dumpType(str_repeat("\xff", 2) === "\xfe\xfe");
    dumpType(str_repeat("\xc3", 2) === "\xc3\xc3");
    dumpType(str_repeat("\xc3", 2) === "\xc2\xc2");
    dumpType(strlen(str_repeat("\xff", 99)));
    dumpType(str_repeat("\xff", 99) === str_repeat("\xfe", 99));
    dumpType(str_repeat("\xff", 100));
    dumpType(str_repeat("\xc3\x80", 49));
    dumpType(str_repeat("\xc3\x80", 50));
    dumpType(str_repeat($flag ? "\xff" : "\xfe", 50));
    dumpType(str_repeat("\0", 99));
    dumpType(str_repeat("\0", 100));
    dumpType(str_repeat($flag ? "\xff" : "\xfe", 2));
    dumpType($flag ? "\xc3" : "\xc2");
    dumpType(987);
}

/** @param "\xff"|"\xfe" $input */
function annotated(string $input): void
{
    dumpType(str_repeat($input, 2));
    dumpType(str_repeat($input, 50));
    dumpType(987);
}

/** @param "\$"|"\Xff"|"\u{d800}" $input */
function phpdocEscapeGrammar(string $input): void
{
    dumpType(str_repeat($input, 2));
    dumpType(str_repeat($input, 16));
    dumpType(str_repeat($input, 17));
    dumpType(987);
}
