<?php

final class ExistingDocOtherParam
{
    /**
     * Runs it.
     *
     * @param int $limit the limit
     */
    public function run(array $items, int $limit): void
    {
        array_map(fn (string $item) => strlen($item), $items);
    }
}
-----
<?php

final class ExistingDocOtherParam
{
    /**
     * Runs it.
     *
     * @param int $limit the limit
     * @param string[] $items
     */
    public function run(array $items, int $limit): void
    {
        array_map(fn (string $item) => strlen($item), $items);
    }
}
