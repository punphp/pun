<?php

final class DummyParam
{
    /**
     * @param array $items the items
     */
    public function run(array $items): void
    {
        array_filter($items, function (int $item) {
            return $item > 1;
        });
    }
}
