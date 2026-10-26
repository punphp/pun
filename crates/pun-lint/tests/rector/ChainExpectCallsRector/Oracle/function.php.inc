<?php

#[Attr]
function &helper(int $a, $b): int
{
    expect($i)->toBe(1);
    expect($i)->toBe(2) ?>
<?php
    expect($i)->toBe(3);
}

class T
{
    #[Attr]
    public static function &m(int $a): int
    {
        expect($i)->toBe(1);
        expect($i)->toBe(2) ?>
<?php
        expect($i)->toBe(3);
    }
}
-----
<?php

#[Attr]
function &helper(int $a, $b): int
{
    expect($i)->toBe(1)
        ->toBe(2)
        ->toBe(3);
}

class T
{
    #[Attr]
    public static function &m(int $a): int
    {
        expect($i)->toBe(1)
            ->toBe(2)
            ->toBe(3);
    }
}
