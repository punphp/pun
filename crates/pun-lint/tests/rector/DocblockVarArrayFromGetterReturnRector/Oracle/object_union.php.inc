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
     * @var \ArrayObject[]|\ArrayIterator[]|\SplStack[]|\SplQueue[]|\SplObjectStorage[]|\SplFixedArray[]
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
