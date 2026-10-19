<?php

/**
 * @param mixed $a
 * @param mixed $b
 * @param mixed ...$c
 * @param mixed &$d
 * @return mixed
 */
function f(int $a, $b, string ...$c, array &$d): int
{
    return 1;
}

class A
{
    /**
     * Run it.
     *
     * @param mixed $a the a
     * @param  mixed  $b
     * @param Mixed $c
     * @param mixed|null $d
     * @return mixed
     */
    public function run(int $a, int $b, int $c, ?int $d)
    {
    }

    /**
     * @return mixed
     * @return mixed
     */
    public function two(): mixed
    {
    }

    /**
     * @return int
     * @return mixed
     */
    public function first(): int
    {
    }

    /**
     * @param mixed $missing
     * @return mixed Some text.
     */
    abstract public function nothing(int $a): mixed;

    /** @param mixed $a */
    public function one(int $a) {}

    /**
     * @param mixed $a
     *
     * More text.
     */
    public function para(int $a) {}
}
-----
<?php

/**
 * @param mixed $b
 */
function f(int $a, $b, string ...$c, array &$d): int
{
    return 1;
}

class A
{
    /**
     * Run it.
     *
     * @param mixed $a the a
     * @param Mixed $c
     * @param mixed|null $d
     * @return mixed
     */
    public function run(int $a, int $b, int $c, ?int $d)
    {
    }

    public function two(): mixed
    {
    }

    /**
     * @return int
     * @return mixed
     */
    public function first(): int
    {
    }

    /**
     * @param mixed $missing
     * @return mixed Some text.
     */
    abstract public function nothing(int $a): mixed;

    public function one(int $a) {}

    /**
     * @param mixed $a
     *
     * More text.
     */
    public function para(int $a) {}
}
