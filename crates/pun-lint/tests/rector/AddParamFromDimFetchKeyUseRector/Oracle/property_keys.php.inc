<?php

final class Keys
{
    /**
     * @var array{a: int, 0: string}
     */
    private array $pairs = ['a' => 1, 0 => 'b'];

    /**
     * @param int|string $key
     */
    public function onProperty($key)
    {
        return $this->pairs[$key];
    }

    /**
     * @param int|string $key
     */
    public function computed($key)
    {
        $map = $this->pairs;

        return $map[$key];
    }
}
