<?php

namespace StaticPropertyOperands;

class Holder
{
    public static int $number = 1;
    public static ?int $nullable = null;
    public static null $nil = null;
    public static string $word = 'word';
    public static bool $flag = false;
    private static int $secret = 1;
    public int $instance = 1;
}

function failed_read(): void
{
    \PHPStan\dumpType(Holder::$missing);
}

function failed_pre_increment(): void { ++Holder::$missing; }
function failed_post_increment(): void { Holder::$missing++; }
function failed_pre_decrement(): void { --Holder::$missing; }
function failed_post_decrement(): void { Holder::$missing--; }

/** @param 'missing'|'other' $name */
function failed_dynamic_read(string $name): void
{
    \PHPStan\dumpType(Holder::${$name});
}

/** @param 'missing'|'other' $name */
function failed_dynamic_pre_increment(string $name): void { ++Holder::${$name}; }
/** @param 'missing'|'other' $name */
function failed_dynamic_post_increment(string $name): void { Holder::${$name}++; }
/** @param 'missing'|'other' $name */
function failed_dynamic_pre_decrement(string $name): void { --Holder::${$name}; }
/** @param 'missing'|'other' $name */
function failed_dynamic_post_decrement(string $name): void { Holder::${$name}--; }

/** @param 'missing'|'nullable' $name */
function partly_missing(string $name): void
{
    \PHPStan\dumpType(Holder::${$name});
    --Holder::${$name};
}

/** @param 'nil'|'other' $name */
function partly_null(string $name): void { Holder::${$name}--; }

/** @param 'number'|'missing' $name */
function partly_number(string $name): void { ++Holder::${$name}; }

function real_null_pre_decrement(): void { --Holder::$nil; }
function real_null_post_decrement(): void { Holder::$nil--; }
function real_null_pre_increment(): void { ++Holder::$nil; }
function real_null_post_increment(): void { Holder::$nil++; }

/** @param 'nil' $name */
function real_dynamic_null(string $name): void { --Holder::${$name}; }

function real_nullable(): void { --Holder::$nullable; }
/** @param 'nullable'|'number' $name */
function real_nullable_union(string $name): void { Holder::${$name}--; }

function valid_direct(): void
{
    \PHPStan\dumpType(++Holder::$number);
    \PHPStan\dumpType(Holder::$number++);
    \PHPStan\dumpType(--Holder::$number);
    \PHPStan\dumpType(Holder::$number--);
}

/** @param 'number' $name */
function valid_dynamic(string $name): void
{
    ++Holder::${$name};
    Holder::${$name}++;
    --Holder::${$name};
    Holder::${$name}--;
}

function unknown_name(string $name): void { --Holder::${$name}; }
function non_string_name(int $name): void { Holder::${$name}--; }
function empty_name(): void { --Holder::${''}; }
function private_name(): void { --Holder::$secret; }
function instance_name(): void { --Holder::$instance; }
function actual_word(): void { --Holder::$word; }
function actual_flag(): void { Holder::$flag++; }
