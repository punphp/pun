<?php

namespace App;

use App\Models\User;

final class MixedIncompatible
{
    /**
     * @return string
     */
    public function a(): int
    {
        return 1;
    }

    /**
     * @return int|string
     */
    public function b(): int
    {
        return 1;
    }

    /**
     * @return User
     */
    public function c(): string
    {
        return '';
    }

    /**
     * @return int[]
     */
    public function d(): string
    {
        return '';
    }

    /**
     * @return positive-int
     */
    public function e(): int
    {
        return 1;
    }

    /**
     * @return null
     */
    public function f(): ?int
    {
        return null;
    }

    /**
     * @return array<string>
     */
    public function g(): iterable
    {
        return [];
    }

    /**
     * Text.
     *
     * @return string the value
     */
    public function h(): int
    {
        return 1;
    }

    /**
     * @return \stdClass
     */
    public function i(): User
    {
        return new User();
    }

    /**
     * @return bool
     */
    public function j(): void
    {
    }

    /**
     * @return float
     */
    public function k(): int
    {
        return 1;
    }

    /**
     * @return int
     */
    public function l(): float
    {
        return 1;
    }
}

/**
 * @return string
 */
function f(): int
{
    return 1;
}
-----
<?php

namespace App;

use App\Models\User;

final class MixedIncompatible
{
    public function a(): int
    {
        return 1;
    }

    /**
     * @return int|string
     */
    public function b(): int
    {
        return 1;
    }

    public function c(): string
    {
        return '';
    }

    public function d(): string
    {
        return '';
    }

    /**
     * @return positive-int
     */
    public function e(): int
    {
        return 1;
    }

    /**
     * @return null
     */
    public function f(): ?int
    {
        return null;
    }

    /**
     * @return array<string>
     */
    public function g(): iterable
    {
        return [];
    }

    /**
     * Text.
     *
     * @return string the value
     */
    public function h(): int
    {
        return 1;
    }

    /**
     * @return \stdClass
     */
    public function i(): User
    {
        return new User();
    }

    public function j(): void
    {
    }

    public function k(): int
    {
        return 1;
    }

    public function l(): float
    {
        return 1;
    }
}

function f(): int
{
    return 1;
}
