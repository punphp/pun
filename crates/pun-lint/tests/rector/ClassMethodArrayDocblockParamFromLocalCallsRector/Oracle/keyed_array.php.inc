<?php

final class KeyedArray
{
    public function go(): void
    {
        $this->run(['a' => 1, 'b' => 2]);
    }

    private function run(array $map): void
    {
    }
}
-----
<?php

final class KeyedArray
{
    public function go(): void
    {
        $this->run(['a' => 1, 'b' => 2]);
    }

    /**
     * @param array<string, int> $map
     */
    private function run(array $map): void
    {
    }
}
