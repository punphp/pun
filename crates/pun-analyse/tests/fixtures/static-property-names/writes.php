<?php declare(strict_types=1);

namespace StaticPropertyWrites;

class Holder
{
    public static null $nil = null;
    public static int $number = 1;
    public static string $word = 'word';
    /** @var 1 */
    public static int $one = 1;
    /** @var '9' */
    public static string $nine = '9';
    private static null $hidden = null;
    public int $instance = 1;
    /** @readonly */
    public static int $frozen = 1;
}

function null_pre(): void { Holder::$nil = ++Holder::$nil; }
function null_post(): void { Holder::$nil = Holder::$nil++; }
function null_pre_dec(): void { Holder::$nil = --Holder::$nil; }
function null_post_dec(): void { Holder::$nil = Holder::$nil--; }
function one_pre(): void { Holder::$one = ++Holder::$one; }
function one_post(): void { Holder::$one = Holder::$one++; }
function one_pre_dec(): void { Holder::$one = --Holder::$one; }
function one_post_dec(): void { Holder::$one = Holder::$one--; }
function string_pre(): void { Holder::$nine = ++Holder::$nine; }
function string_post(): void { Holder::$nine = Holder::$nine++; }
function string_pre_dec(): void { Holder::$nine = --Holder::$nine; }
function string_post_dec(): void { Holder::$nine = Holder::$nine--; }

/** @param 'nil' $name */
function dynamic_pre(string $name): void { ++Holder::${$name}; }
/** @param 'nil' $name */
function dynamic_post(string $name): void { Holder::${$name}++; }
/** @param 'nil'|'number' $name */
function finite_pre(string $name): void { ++Holder::${$name}; }
/** @param 'nil'|'number' $name */
function finite_post(string $name): void { Holder::${$name}++; }
/** @param 'nil'|'missing' $name */
function failed_alternative_pre(string $name): void { ++Holder::${$name}; }
/** @param 'nil'|'missing' $name */
function failed_alternative_post(string $name): void { Holder::${$name}++; }
function failed_pre(): void { ++Holder::$missing; }
function failed_post(): void { Holder::$missing--; }
function unknown_pre(string $name): void { ++Holder::${$name}; }
function unknown_post(string $name): void { Holder::${$name}--; }
function unknown_receiver_pre(object $holder): void { ++$holder::$nil; }
function unknown_receiver_post(object $holder): void { $holder::$nil--; }
function invalid_receiver_pre(int $holder): void { ++$holder::$nil; }
function invalid_receiver_post(int $holder): void { $holder::$nil--; }
function private_pre(): void { ++Holder::$hidden; }
function private_post(): void { Holder::$hidden++; }
function private_pre_dec(): void { --Holder::$hidden; }
function private_post_dec(): void { Holder::$hidden--; }
function instance_pre(): void { ++Holder::$instance; }
function instance_post(): void { Holder::$instance--; }
function readonly_pre(): void { ++Holder::$frozen; }
function readonly_post(): void { Holder::$frozen--; }

function valid(): void
{
    ++Holder::$number;
    Holder::$number++;
    --Holder::$number;
    Holder::$number--;
    ++Holder::$word;
    Holder::$word++;
    --Holder::$word;
    Holder::$word--;
}

class InstanceHolder
{
    public readonly int $locked;
    public function __construct(int $number) { $this->locked = $number; }
    public function prefix(): void { ++$this->locked; }
    public function postfix(): void { $this->locked--; }
}

function readonly_receiver_pre(InstanceHolder $holder): void { ++$holder->locked; }
function readonly_receiver_post(InstanceHolder $holder): void { $holder->locked--; }

/** @param 'nil'|'word' $name */
function finite_string_pre(string $name): void { ++Holder::${$name}; }
/** @param 'nil'|'word' $name */
function finite_string_post(string $name): void { Holder::${$name}--; }
/** @param 'one'|'number' $name */
function finite_literal_pre(string $name): void { ++Holder::${$name}; }
/** @param 'one'|'number' $name */
function finite_literal_post(string $name): void { Holder::${$name}++; }

function multiline_pre(): void
{
    ++
        Holder::$nil;
}

function multiline_post(): void
{
    Holder::$nil
        ++;
}

class OtherHolder
{
    public static null $nil = null;
}

function union_receiver(Holder|OtherHolder $holder): void
{
    $name = 'nil';
    ++$holder::${$name};
}

class StringHolder
{
    /** @var literal-string */
    public static string $literal = 'word';
    /** @var numeric-string */
    public static string $numeric = '9';
}

function literal_pre(): void { ++StringHolder::$literal; }
function literal_post(): void { StringHolder::$literal++; }
function literal_pre_dec(): void { --StringHolder::$literal; }
function literal_post_dec(): void { StringHolder::$literal--; }
function numeric_pre(): void { ++StringHolder::$numeric; }
function numeric_post(): void { StringHolder::$numeric++; }
function numeric_pre_dec(): void { --StringHolder::$numeric; }
function numeric_post_dec(): void { StringHolder::$numeric--; }

function invalid_callable_pre(int $callback): void { ++$callback(); }
function invalid_callable_post(int $callback): void { $callback()--; }
