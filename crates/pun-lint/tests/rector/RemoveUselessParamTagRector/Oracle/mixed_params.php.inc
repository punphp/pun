<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedParams
{
    /**
     * @param int $a
     * @param int[] $b
     * @param User $c
     * @param When $d
     * @param string $e the e
     * @param int|null $f
     * @param non-empty-string $g
     * @param string|int $h
     * @param $i
     * @param \App\Models\User $j
     */
    public function run(int $a, array $b, User $c, When $d, string $e, ?int $f, string $g, int|string $h, int $i, User $j): void
    {
    }

    /**
     * Text.
     *
     * @param int $a
     * @return void
     */
    public function described(int $a): void
    {
    }

    /**
     * @param int $a
     */
    // between
    public function noted(int $a): void
    {
    }

    /**
     * @param int $a
     * @param int $a
     */
    public function twice(int $a): void
    {
    }

    /**
     * @param int $missing
     */
    public function missing(int $a): void
    {
    }

    /**
     * @param int ...$rest
     */
    public function variadic(int ...$rest): void
    {
    }

    /**
     * @param int &$ref
     */
    public function reference(int &$ref): void
    {
    }

    /**
     * @param self $other
     * @param static $same
     */
    public function selfish(self $other, self $same): void
    {
    }

    /**
     * @param mixed $m
     * @param bool $b
     * @param false $f
     */
    public function scalars(mixed $m, bool $b, bool $f): void
    {
    }
}

/**
 * @param int $a
 */
$closure = function (int $a): void {
};
-----
<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedParams
{
    /**
     * @param int[] $b
     * @param string $e the e
     * @param non-empty-string $g
     * @param $i
     */
    public function run(int $a, array $b, User $c, When $d, string $e, ?int $f, string $g, int|string $h, int $i, User $j): void
    {
    }

    /**
     * Text.
     *
     * @return void
     */
    public function described(int $a): void
    {
    }

    // between
    public function noted(int $a): void
    {
    }

    public function twice(int $a): void
    {
    }

    /**
     * @param int $missing
     */
    public function missing(int $a): void
    {
    }

    public function variadic(int ...$rest): void
    {
    }

    public function reference(int &$ref): void
    {
    }

    public function selfish(self $other, self $same): void
    {
    }

    /**
     * @param false $f
     */
    public function scalars(mixed $m, bool $b, bool $f): void
    {
    }
}

/**
 * @param int $a
 */
$closure = function (int $a): void {
};
