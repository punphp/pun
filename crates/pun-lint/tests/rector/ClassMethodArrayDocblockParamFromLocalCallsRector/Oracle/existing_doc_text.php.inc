<?php

final class ExistingDocText
{
    public function go(): void
    {
        $this->run([1.5, 2.5], 3);
    }

    /**
     * Runs the items.
     *
     * @param int $limit the limit
     */
    private function run(array $items, int $limit): void
    {
    }
}
-----
<?php

final class ExistingDocText
{
    public function go(): void
    {
        $this->run([1.5, 2.5], 3);
    }

    /**
     * Runs the items.
     *
     * @param int $limit the limit
     * @param float[] $items
     */
    private function run(array $items, int $limit): void
    {
    }
}
