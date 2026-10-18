<?php

if ($a) {
    $x = 1;
}
return $x;

class InMethod
{
    public function run($a)
    {
        if ($a) {
            $x = 1;
        }
        return $x;
    }
}

$f = function ($a) {
    if ($a) {
        $x = 1;
    }
    return $x;
};

function emptyVarTag($a)
{
    if ($a) {
        /** @var */
        $x = 1;
    }
    return $x;
}

function emptyVarTagLine($a)
{
    if ($a) {
        /**
         * @var
         */
        $x = 1;
    }
    return $x;
}

$g = fn (&$x) => function () {
    if ($this->a) {
        $x = 1;
    }
    return $x;
};
-----
<?php

if ($a) {
    return 1;
}
return $x;

class InMethod
{
    public function run($a)
    {
        if ($a) {
            return 1;
        }
        return $x;
    }
}

$f = function ($a) {
    if ($a) {
        return 1;
    }
    return $x;
};

function emptyVarTag($a)
{
    if ($a) {
        /** @var */
        return 1;
    }
    return $x;
}

function emptyVarTagLine($a)
{
    if ($a) {
        /**
         * @var
         */
        return 1;
    }
    return $x;
}

$g = fn (&$x) => function () {
    if ($this->a) {
        $x = 1;
    }
    return $x;
};
