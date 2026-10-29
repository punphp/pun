<?php

namespace Probe;

class Checks
{
    /**
     * @phpstan-assert array $actual
     */
    public function assertIsArray(mixed $actual): void
    {
    }

    public function assertSame(mixed $expected, mixed $actual): void
    {
    }

    /**
     * @template TValue
     *
     * @param  TValue  $value
     * @param  callable(TValue): mixed  $callback
     */
    public function tap($value, callable $callback): void
    {
    }

    public function asserted($value)
    {
        $this->tap($value->toArray(), function ($rows) {
            $this->assertIsArray($rows);
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function notAsserted($value)
    {
        $this->tap($value->toArray(), function ($rows) {
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function otherCall($value)
    {
        $this->tap($value->toArray(), function ($rows) {
            $this->assertSame([], $rows);
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }
}
-----
<?php

namespace Probe;

class Checks
{
    /**
     * @phpstan-assert array $actual
     */
    public function assertIsArray(mixed $actual): void
    {
    }

    public function assertSame(mixed $expected, mixed $actual): void
    {
    }

    /**
     * @template TValue
     *
     * @param  TValue  $value
     * @param  callable(TValue): mixed  $callback
     */
    public function tap($value, callable $callback): void
    {
    }

    public function asserted($value)
    {
        $this->tap($value->toArray(), function (array $rows) {
            $this->assertIsArray($rows);
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function notAsserted($value)
    {
        $this->tap($value->toArray(), function ($rows) {
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function otherCall($value)
    {
        $this->tap($value->toArray(), function ($rows) {
            $this->assertSame([], $rows);
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }
}
