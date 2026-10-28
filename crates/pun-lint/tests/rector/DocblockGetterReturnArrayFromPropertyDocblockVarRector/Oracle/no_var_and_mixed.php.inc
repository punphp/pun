<?php

final class NoVarAndMixed
{
    /** The items. */
    private array $items = [];

    /** @var mixed[] */
    private array $mixed = [];

    /** @var string */
    private array $plain = [];

    public function getItems(): array
    {
        return $this->items;
    }

    public function getMixed(): array
    {
        return $this->mixed;
    }

    public function getPlain(): array
    {
        return $this->plain;
    }
}
-----
<?php

final class NoVarAndMixed
{
    /** The items. */
    private array $items = [];

    /** @var mixed[] */
    private array $mixed = [];

    /** @var string */
    private array $plain = [];

    public function getItems(): array
    {
        return $this->items;
    }

    /**
     * @return mixed[]
     */
    public function getMixed(): array
    {
        return $this->mixed;
    }

    public function getPlain(): array
    {
        return $this->plain;
    }
}
