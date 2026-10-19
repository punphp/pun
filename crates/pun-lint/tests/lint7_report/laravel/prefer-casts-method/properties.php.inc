<?php

class User
{
    protected $casts = [
        'options' => 'array',
    ];
}

class Legacy
{
    protected $fillable = [], $casts = array('a' => 'b');
}

class Twice
{
    public $casts = [], $other = [];
}

class NotArray
{
    protected $casts;
    protected $CASTS = [];
    protected ?array $typed = null;
}

class Registry
{
    protected static $casts = ['a' => 'b'];
}

class Hooked
{
    public array $casts = [] {
        get => $this->casts;
    }
}

trait T
{
    protected $casts = [];
}

$x = new class {
    protected $casts = ['x' => 'y'];
};
-----
5:15: Define model casts in the `casts()` method instead of the `$casts` property.
12:31: Define model casts in the `casts()` method instead of the `$casts` property.
17:12: Define model casts in the `casts()` method instead of the `$casts` property.
41:15: Define model casts in the `casts()` method instead of the `$casts` property.
45:15: Define model casts in the `casts()` method instead of the `$casts` property.
