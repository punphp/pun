<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, class-string<\A>|class-string<\B>|class-string<\C>|class-string<\D>>
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
     * @var array<string, class-string>
     */
    private array $items = [];

    /**
     * @return array<string, class-string<\A>|class-string<\B>|class-string<\C>|class-string<\D>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
