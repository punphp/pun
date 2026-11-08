<?php

namespace Oracle\ExplicitNull3;

/**
 * @return never
 */
function stop()
{
    throw new \RuntimeException();
}

final class Runner
{
    public function a(int $n)
    {
        if ($n) {
            return 1;
        }
        stop();
    }

    /**
     * @return int|void
     * @return string
     */
    public function b(int $n)
    {
        if ($n) {
            return 1;
        }
    }

    /** @return int |void */
    public function c(int $n)
    {
        if ($n) {
            return 1;
        }
    }

    /**
     * @return
     */
    public function d(int $n)
    {
        if ($n) {
            return 1;
        }
    }

    /** @return Sub\Thing|void */
    public function e(int $n)
    {
        if ($n) {
            return 1;
        }
    }

    /** @return true|false|bool|void */
    public function f(int $n)
    {
        if ($n) {
            return 1;
        }
    }

    /**
     * Without a return tag.
     */
    public function h(int $n)
    {
        if ($n) {
            return 1;
        }
    }
}

trait Shared
{
    /** @return self|void */
    public function a(int $n)
    {
        if ($n) {
            return 1;
        }
    }
}

enum Suit
{
    case A;

    /** @return self|void */
    public function b(int $n)
    {
        if ($n) {
            return 1;
        }
    }
}

/** @return self|void */
function outside(int $n)
{
    if ($n) {
        return 1;
    }
}
-----
<?php

namespace Oracle\ExplicitNull3;

/**
 * @return never
 */
function stop()
{
    throw new \RuntimeException();
}

final class Runner
{
    public function a(int $n)
    {
        if ($n) {
            return 1;
        }
        stop();
    }

    /**
     * @return int|null
     * @return string
     */
    public function b(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }

    /** @return int|null */
    public function c(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }

    /**
     * @return
     */
    public function d(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }

    /** @return Sub\Thing|null */
    public function e(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }

    /** @return bool|null */
    public function f(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }

    /**
     * Without a return tag.
     */
    public function h(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }
}

trait Shared
{
    /** @return \Oracle\ExplicitNull3\Shared|null */
    public function a(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }
}

enum Suit
{
    case A;

    /** @return \Oracle\ExplicitNull3\Suit|null */
    public function b(int $n)
    {
        if ($n) {
            return 1;
        }
        return null;
    }
}

/** @return self|void */
function outside(int $n)
{
    if ($n) {
        return 1;
    }
    return null;
}
