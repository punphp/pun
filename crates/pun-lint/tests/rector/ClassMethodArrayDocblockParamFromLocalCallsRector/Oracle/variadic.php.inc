<?php

final class Variadic
{
    public function go(): void
    {
        $this->run(['a'], ['b']);
    }

    private function run(array ...$lists): void
    {
    }
}
-----
<?php

final class Variadic
{
    public function go(): void
    {
        $this->run(['a'], ['b']);
    }

    /**
     * @param string[] ...$lists
     */
    private function run(array ...$lists): void
    {
    }
}
