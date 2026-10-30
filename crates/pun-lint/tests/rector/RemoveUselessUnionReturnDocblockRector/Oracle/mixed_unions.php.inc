<?php

namespace App;

use App\Models\User;

final class MixedUnions
{
    /**
     * @return int|string
     */
    public function a(): int|string
    {
        return 1;
    }

    /**
     * @return string|int
     */
    public function b(): int|string
    {
        return 1;
    }

    /**
     * @return int|string|null
     */
    public function c(): int|string|null
    {
        return 1;
    }

    /**
     * @return User|string
     */
    public function d(): User|string
    {
        return '';
    }

    /**
     * @return int[]|string
     */
    public function e(): array|string
    {
        return '';
    }

    /**
     * @return int|string the value
     */
    public function f(): int|string
    {
        return 1;
    }

    /**
     * @return int|null
     */
    public function g(): ?int
    {
        return 1;
    }

    /**
     * @return int|float|string
     */
    public function h(): int|string
    {
        return 1;
    }

    /**
     * Text.
     *
     * @return int|string
     */
    public function i(): int|string
    {
        return 1;
    }

    /**
     * @return \stdClass|User
     */
    public function j(): User|\stdClass
    {
        return new User();
    }
}
-----
<?php

namespace App;

use App\Models\User;

final class MixedUnions
{
    public function a(): int|string
    {
        return 1;
    }

    public function b(): int|string
    {
        return 1;
    }

    public function c(): int|string|null
    {
        return 1;
    }

    public function d(): User|string
    {
        return '';
    }

    /**
     * @return int[]|string
     */
    public function e(): array|string
    {
        return '';
    }

    /**
     * @return int|string the value
     */
    public function f(): int|string
    {
        return 1;
    }

    public function g(): ?int
    {
        return 1;
    }

    public function h(): int|string
    {
        return 1;
    }

    /**
     * Text.
     */
    public function i(): int|string
    {
        return 1;
    }

    public function j(): User|\stdClass
    {
        return new User();
    }
}
