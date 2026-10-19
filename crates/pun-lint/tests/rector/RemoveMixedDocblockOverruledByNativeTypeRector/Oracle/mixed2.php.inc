<?php

class A
{
    /**
     * Determine if the given value is in the given stack.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  bool  $strict
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function stackContains(string $key, mixed $value, bool $strict = false): bool
    {
    }

    /**
     * Text.
     *
     * @param  mixed  $value
     *
     * @throws \RuntimeException
     */
    public function two(mixed $value): bool
    {
    }

    /**
     * @param  mixed  $value
     *
     * @throws \RuntimeException
     */
    public function three(mixed $value): bool
    {
    }
}
-----
<?php

class A
{
    /**
     * Determine if the given value is in the given stack.
     *
     * @param  string  $key
     * @param  bool  $strict
     * @return bool
     * @throws \RuntimeException
     */
    public function stackContains(string $key, mixed $value, bool $strict = false): bool
    {
    }

    /**
     * Text.
     *
     *
     * @throws \RuntimeException
     */
    public function two(mixed $value): bool
    {
    }

    /**
     * @throws \RuntimeException
     */
    public function three(mixed $value): bool
    {
    }
}
