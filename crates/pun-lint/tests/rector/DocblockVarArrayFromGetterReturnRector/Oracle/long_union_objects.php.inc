<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, \SomeVeryLongClassNameNumberOne>|array<string, \SomeVeryLongClassNameNumberTwo>|array<string, \SomeVeryLongClassNameNumberThree>
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
     * @var array<string, mixed>
     */
    private array $items = [];

    /**
     * @return array<string, \SomeVeryLongClassNameNumberOne>|array<string, \SomeVeryLongClassNameNumberTwo>|array<string, \SomeVeryLongClassNameNumberThree>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
