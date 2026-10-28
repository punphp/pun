<?php

final class Keys
{
    /**
     * @var array<string, array<string, int>>
     */
    private array $nested = [];

    /**
     * @var array<int, string>
     */
    private array $dynamic = [];

    public function nested($key)
    {
        $this->nested[$key]['count'] = 1;
    }

    public function dynamic($key, string $name)
    {
        $this->{$name}[$key] = 'x';
    }

    public function empty($key)
    {
        $items = [];

        return $items[$key];
    }

    public function legacy($key)
    {
        $map = array('a' => 1, 2 => 'b');

        return $map[$key];
    }

    public function unkeyed($key)
    {
        $map = ['a' => 1, 'b'];

        return $map[$key];
    }
}
-----
<?php

final class Keys
{
    /**
     * @var array<string, array<string, int>>
     */
    private array $nested = [];

    /**
     * @var array<int, string>
     */
    private array $dynamic = [];

    public function nested(string $key)
    {
        $this->nested[$key]['count'] = 1;
    }

    public function dynamic($key, string $name)
    {
        $this->{$name}[$key] = 'x';
    }

    public function empty($key)
    {
        $items = [];

        return $items[$key];
    }

    public function legacy(string|int $key)
    {
        $map = array('a' => 1, 2 => 'b');

        return $map[$key];
    }

    public function unkeyed(string|int $key)
    {
        $map = ['a' => 1, 'b'];

        return $map[$key];
    }
}
