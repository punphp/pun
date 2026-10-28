<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedReturns
{
    /**
     * @return int
     */
    public function a(): int
    {
        return 1;
    }

    /**
     * @return int[]
     */
    public function b(): array
    {
        return [];
    }

    /**
     * @return User
     */
    public function c(): User
    {
        return new User();
    }

    /**
     * @return When
     */
    public function d(): When
    {
    }

    /**
     * @return string the name
     */
    public function e(): string
    {
        return '';
    }

    /**
     * @return int|null
     */
    public function f(): ?int
    {
        return null;
    }

    /**
     * @return non-empty-string
     */
    public function g(): string
    {
        return 'g';
    }

    /**
     * @return string|int
     */
    public function h(): int|string
    {
        return 1;
    }

    /**
     * @return self
     */
    public function i(): self
    {
        return $this;
    }

    /**
     * @return static
     */
    public function j(): static
    {
        return $this;
    }

    /**
     * @return $this
     */
    public function k(): self
    {
        return $this;
    }

    /**
     * @return void
     */
    public function l(): void
    {
    }

    /**
     * @return never
     */
    public function m(): never
    {
        throw new \Exception();
    }

    /**
     * @return \Generator
     */
    public function n(): \Generator
    {
        yield 1;
    }

    /**
     * @return iterable
     */
    public function o(): iterable
    {
        return [];
    }

    /**
     * @return bool
     */
    public function p(): bool
    {
        return true;
    }

    /**
     * @return true
     */
    public function q(): bool
    {
        return true;
    }

    /**
     * @return int
     */
    // note
    public function r(): int
    {
        return 1;
    }

    /**
     * @return object
     */
    public function s(): object
    {
        return $this;
    }

    /**
     * @return callable
     */
    public function t(): callable
    {
        return 'strlen';
    }

    /**
     * @return float
     */
    public function u(): float
    {
        return 1.0;
    }
}
-----
<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedReturns
{
    public function a(): int
    {
        return 1;
    }

    /**
     * @return int[]
     */
    public function b(): array
    {
        return [];
    }

    public function c(): User
    {
        return new User();
    }

    public function d(): When
    {
    }

    /**
     * @return string the name
     */
    public function e(): string
    {
        return '';
    }

    public function f(): ?int
    {
        return null;
    }

    /**
     * @return non-empty-string
     */
    public function g(): string
    {
        return 'g';
    }

    public function h(): int|string
    {
        return 1;
    }

    public function i(): self
    {
        return $this;
    }

    public function j(): static
    {
        return $this;
    }

    /**
     * @return $this
     */
    public function k(): self
    {
        return $this;
    }

    public function l(): void
    {
    }

    public function m(): never
    {
        throw new \Exception();
    }

    public function n(): \Generator
    {
        yield 1;
    }

    public function o(): iterable
    {
        return [];
    }

    public function p(): bool
    {
        return true;
    }

    /**
     * @return true
     */
    public function q(): bool
    {
        return true;
    }

    // note
    public function r(): int
    {
        return 1;
    }

    public function s(): object
    {
        return $this;
    }

    public function t(): callable
    {
        return 'strlen';
    }

    public function u(): float
    {
        return 1.0;
    }
}
