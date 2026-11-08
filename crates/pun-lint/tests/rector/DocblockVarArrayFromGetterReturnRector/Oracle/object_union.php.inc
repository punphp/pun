<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, \ArrayObject|\ArrayIterator|\SplStack|\SplQueue|\SplObjectStorage|\SplFixedArray>
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
     * @var \ArrayIterator[]|\ArrayObject[]|\SplFixedArray[]|\SplObjectStorage[]|\SplQueue[]|\SplStack[]
     */
    private array $items = [];

    /**
     * @return array<int, \ArrayObject|\ArrayIterator|\SplStack|\SplQueue|\SplObjectStorage|\SplFixedArray>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
