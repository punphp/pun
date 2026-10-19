<?php

/**
 * @param null $a
 * @param null $b described
 * @param null $missing
 * @param int $c
 * @phpstan-param int|null $a
 * @return null
 * @phpstan-return null
 */
function f($a, $b, $c)
{
    /** @var null $x */
    $x = g();
    /** @var null */
    $y = g();
    /** @var null $z with words */
    $z = g();
    /**
     * @var null
     * @phan-var null
     * @psalm-var null
     */
    $w = g();
    /** @var NULL */
    $v = g();
    /** @var null|int */
    $u = g();
    // plain
    /** @var null */
    // after
    $t = g();
    /** @var null */
    return $t;
}

/**
 * Some text.
 *
 * @return null
 */
function text()
{
}

/**
 * @return null
 *   continued
 */
function continued()
{
}

/**
 * @return int
 * @return null
 */
function second()
{
}

/**
 * @param int $a
 * @param null $a
 */
function first_param($a)
{
}

/**
 * @param null &$a
 * @param null ...$b
 */
function by_ref(&$a, ...$b)
{
}

interface I
{
    /**
     * @return null
     */
    public function m();
}

class C
{
    /** @var null */
    public $p;

    /**
     * @var null $q
     */
    public $q, $r;

    #[Attr]
    /**
     * @return null
     */
    public function attributed()
    {
    }

    /**
     * @return null
     */
    #[Attr]
    public function before()
    {
        $c = function () {
            /** @var null */
            $in = 1;
        };
        /** @return null */
        $d = function () {};
    }
}
-----
<?php

/**
 * @param null $b described
 * @param null $missing
 * @param int $c
 * @phpstan-return null
 */
function f($a, $b, $c)
{
    $x = g();
    $y = g();
    /** @var null $z with words */
    $z = g();
    /**
     * @psalm-var null
     */
    $w = g();
    /** @var NULL */
    $v = g();
    /** @var null|int */
    $u = g();
    // plain
    // after
    $t = g();
    /** @var null */
    return $t;
}

/**
 * Some text.
 */
function text()
{
}

/**
 * @return null
 *   continued
 */
function continued()
{
}

/**
 * @return int
 * @return null
 */
function second()
{
}

/**
 * @param int $a
 * @param null $a
 */
function first_param($a)
{
}

function by_ref(&$a, ...$b)
{
}

interface I
{
    public function m();
}

class C
{
    public $p;

    public $q, $r;

    #[Attr]
    /**
     * @return null
     */
    public function attributed()
    {
    }

    #[Attr]
    public function before()
    {
        $c = function () {
            $in = 1;
        };
        /** @return null */
        $d = function () {};
    }
}
