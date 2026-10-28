<?php

final class Keys
{
    /**
     * @var array{a: int, 0: string}
     */
    private array $pairs = ['a' => 1, 0 => 'b'];

    /**
     * @var array<string, int>
     */
    public array $items = [];

    public function onProperty($key)
    {
        if (is_int($key) || is_string($key)) {
            return $this->pairs[$key];
        }
    }

    public function computed($key)
    {
        $map = $this->pairs;
        if (is_int($key) || is_string($key)) {
            return $map[$key];
        }
    }

    public function other($key, self $other)
    {
        $other->items[$key] = 1;
    }
}
-----
<?php

final class Keys
{
    /**
     * @var array{a: int, 0: string}
     */
    private array $pairs = ['a' => 1, 0 => 'b'];

    /**
     * @var array<string, int>
     */
    public array $items = [];

    public function onProperty($key)
    {
        if (is_int($key) || is_string($key)) {
            return $this->pairs[$key];
        }
    }

    public function computed($key)
    {
        $map = $this->pairs;
        if (is_int($key) || is_string($key)) {
            return $map[$key];
        }
    }

    public function other(string $key, self $other)
    {
        $other->items[$key] = 1;
    }
}
