<?php

final class BoolValues
{
    public function go(): void
    {
        $this->run([true, false]);
    }

    private function run(array $flags): void
    {
    }
}
-----
<?php

final class BoolValues
{
    public function go(): void
    {
        $this->run([true, false]);
    }

    /**
     * @param bool[] $flags
     */
    private function run(array $flags): void
    {
    }
}
