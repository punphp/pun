<?php

final class CallInClosure
{
    public function go(): void
    {
        $fn = function () {
            $this->run(['a']);
        };
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

final class CallInClosure
{
    public function go(): void
    {
        $fn = function () {
            $this->run(['a']);
        };
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
