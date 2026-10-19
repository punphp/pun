<?php

class A
{
    public $property;

    public function a(): bool
    {
        return true;
    }

    public static function s(): bool
    {
        return true;
    }

    public function run($m)
    {
        $r = self::s() === true;
        $r = static::s() === true;
        $r = $this->$m() === true;
        $r = $this->property === true;
        $r = parent::class === true;
    }
}

enum E
{
    public function a(): bool
    {
        return true;
    }

    public function run()
    {
        return $this->a() === true;
    }
}

$anonymous = new class {
    public function a(): bool
    {
        return true;
    }

    public function run()
    {
        return $this->a() === true;
    }
};

function outside()
{
    return $this->a() === true;
}

function param(bool $b)
{
    return $b === true;
}

function written(bool $b)
{
    $b = foo();
    return $b === true;
}

$closure = function (bool $b) {
    return $b === true;
};

$arrow = fn (bool $b) => $b === true;

$r = $undefined === true;

$outer = foo();
$used = function () use ($outer) {
    return $outer === true;
};

function locals($other, $list)
{
    $b = true;
    $c = &$other;
    foreach ($list as $item) {
    }
    return $b === true;
}

function by_reference(bool &$b)
{
    return $b === true;
}
-----
<?php

class A
{
    public $property;

    public function a(): bool
    {
        return true;
    }

    public static function s(): bool
    {
        return true;
    }

    public function run($m)
    {
        $r = self::s();
        $r = static::s();
        $r = $this->$m() === true;
        $r = $this->property === true;
        $r = parent::class === true;
    }
}

enum E
{
    public function a(): bool
    {
        return true;
    }

    public function run()
    {
        return $this->a();
    }
}

$anonymous = new class {
    public function a(): bool
    {
        return true;
    }

    public function run()
    {
        return $this->a();
    }
};

function outside()
{
    return $this->a() === true;
}

function param(bool $b)
{
    return $b;
}

function written(bool $b)
{
    $b = foo();
    return $b === true;
}

$closure = function (bool $b) {
    return $b;
};

$arrow = fn (bool $b) => $b;

$r = $undefined === true;

$outer = foo();
$used = function () use ($outer) {
    return $outer === true;
};

function locals($other, $list)
{
    $b = true;
    $c = &$other;
    foreach ($list as $item) {
    }
    return $b;
}

function by_reference(bool &$b)
{
    return $b;
}
