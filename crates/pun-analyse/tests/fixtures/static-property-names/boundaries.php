<?php

namespace StaticPropertyNames;

/** @param 'present'|'missing' $name */
function write_scope(string $name): void
{
    Holder::${$name} = 1;
    echo Holder::${$name};
    echo Holder::$$name;
    $other = 'missing';
    echo Holder::${$other};
    $name = 'changed';
    echo Holder::${$name};
    echo Holder::$missing;
}

/** @param 'present'|'missing' $name */
function branch_write(bool $condition, string $name): void
{
    if ($condition) {
        Holder::${$name} = 1;
    }
    echo Holder::${$name};
}

/** @param 'present'|'missing' $name */
function guard_scope(string $name): void
{
    if (isset(Holder::${$name})) {
        echo Holder::${$name};
        $other = 'missing';
        echo Holder::${$other};
        $name = 'changed';
        echo Holder::${$name};
    }
    echo Holder::${$name};
}

/** @param 'missing'|'other' $name */
function compound(string $name): void
{
    Holder::${$name} += 1;
}

/** @param 'missing'|'other' $name */
function offset_write(string $name): void
{
    Holder::${$name}[] = 1;
}

/** @param 'missing'|'other' $name */
function increment(string $name): void
{
    Holder::${$name}++;
}

/** @param 'missing'|'other' $name */
function pre_decrement(string $name): void
{
    --Holder::${$name};
}

function multiline(Holder $holder): void
{
    echo Holder::${
        'missing'
    };
    echo $holder::${
        'missing'
    };
    Holder::${
        'missing'
    } = 1;
}

function literal_alias(): void
{
    Holder::${'missing'} = 1;
    echo Holder::$missing;
    echo Holder::${('missing')};
    echo Holder::${'miss' . 'ing'};
}

function literal_is_not_a_dependency(): void
{
    Holder::${'$name'} = 1;
    $name = 'unrelated';
    echo Holder::${'$name'};
}
