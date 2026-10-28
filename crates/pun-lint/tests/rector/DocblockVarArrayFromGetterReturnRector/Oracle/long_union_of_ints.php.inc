<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, int>|array<int, string>|array<string, float>|array<int, bool>|array<string, string>|array<int, int>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var array<mixed[], mixed>
     */
    private array $items = [];

    /**
     * @return array<string, int>|array<int, string>|array<string, float>|array<int, bool>|array<string, string>|array<int, int>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
