<?php

final class MultilineDocs
{
    /**
     * @var array<
     *     int
     * >
     */
    private array $items = [];

    /** @var int[] */
    private array $ids = [];

    public function getItems(): array
    {
        return $this->items;
    }

    /**
     * @return array<
     *     int
     * >
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
-----
<?php

final class MultilineDocs
{
    /**
     * @var array<
     *     int
     * >
     */
    private array $items = [];

    /** @var int[] */
    private array $ids = [];

    /**
     * @return array<int>
     */
    public function getItems(): array
    {
        return $this->items;
    }

    /**
     * @return array<
     *     int
     * >
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
