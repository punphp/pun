<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (?\DateTime $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param \DateTime[]|null[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (?\DateTime $item) => $item, $items);
    }
}
