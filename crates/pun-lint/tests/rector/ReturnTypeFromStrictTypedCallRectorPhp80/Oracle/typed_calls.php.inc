<?php

namespace App;

class Base
{
    public function make(): static
    {
        return $this;
    }

    public function me(): self
    {
        return $this;
    }
}

final class TypedProbe extends Base
{
    const NAME = 'a';

    public function a()
    {
        return $this->make();
    }

    public function b()
    {
        return $this->me();
    }

    public function c()
    {
        return $this->inner();
    }

    public function d($x)
    {
        if ($x) {
            return $this->inner();
        }
        return $this->nullable();
    }

    public function e($x)
    {
        if ($x) {
            return self::NAME;
        }
        return $this->inner();
    }

    public function f($x)
    {
        if ($x) {
            return [];
        }
        return $this->items();
    }

    public function g($x)
    {
        if ($x) {
            return strlen($x);
        }
        return $this->inner();
    }

    public function h($x)
    {
        if ($x) {
            return null;
        }
        return $this->inner();
    }

    public function i($x)
    {
        if ($x) {
            return $this->mixedOne();
        }
        return $this->inner();
    }

    public function j($x)
    {
        return $this->untyped();
    }

    public function k($x)
    {
        return pathinfo($x);
    }

    public function l(\DateTimeImmutable $d)
    {
        return $d->modify('+1 day');
    }

    public function m($x)
    {
        if ($x) {
            return $this->union();
        }
        return $this->inner();
    }

    private function inner(): string
    {
        return '';
    }

    private function nullable(): ?int
    {
        return null;
    }

    private function items(): array
    {
        return [];
    }

    private function mixedOne(): mixed
    {
        return 1;
    }

    private function untyped()
    {
        return 1;
    }

    private function union(): int|array
    {
        return 1;
    }
}
-----
<?php

namespace App;

class Base
{
    public function make(): static
    {
        return $this;
    }

    public function me(): self
    {
        return $this;
    }
}

final class TypedProbe extends Base
{
    const NAME = 'a';

    public function a(): \App\Base
    {
        return $this->make();
    }

    public function b(): \App\Base
    {
        return $this->me();
    }

    public function c(): string
    {
        return $this->inner();
    }

    public function d($x): string|int|null
    {
        if ($x) {
            return $this->inner();
        }
        return $this->nullable();
    }

    public function e($x): string
    {
        if ($x) {
            return self::NAME;
        }
        return $this->inner();
    }

    public function f($x): array
    {
        if ($x) {
            return [];
        }
        return $this->items();
    }

    public function g($x): int|string
    {
        if ($x) {
            return strlen($x);
        }
        return $this->inner();
    }

    public function h($x)
    {
        if ($x) {
            return null;
        }
        return $this->inner();
    }

    public function i($x)
    {
        if ($x) {
            return $this->mixedOne();
        }
        return $this->inner();
    }

    public function j($x)
    {
        return $this->untyped();
    }

    public function k($x): string|array
    {
        return pathinfo($x);
    }

    public function l(\DateTimeImmutable $d): \DateTimeImmutable
    {
        return $d->modify('+1 day');
    }

    public function m($x): int|array|string
    {
        if ($x) {
            return $this->union();
        }
        return $this->inner();
    }

    private function inner(): string
    {
        return '';
    }

    private function nullable(): ?int
    {
        return null;
    }

    private function items(): array
    {
        return [];
    }

    private function mixedOne(): mixed
    {
        return 1;
    }

    private function untyped()
    {
        return 1;
    }

    private function union(): int|array
    {
        return 1;
    }
}
