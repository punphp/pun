<?php

final class NestedArray
{
    public function go(): void
    {
        $this->run([['x' => 1], ['x' => 2]]);
    }

    private function run(array $rows): void
    {
    }
}
-----
<?php

final class NestedArray
{
    public function go(): void
    {
        $this->run([['x' => 1], ['x' => 2]]);
    }

    /**
     * @param array<int, array<string, int>> $rows
     */
    private function run(array $rows): void
    {
    }
}
