<?php

final class SomeClass
{
    public function run(): array
    {
        return [$this, $this];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @return \SomeClass[]
     */
    public function run(): array
    {
        return [$this, $this];
    }
}
