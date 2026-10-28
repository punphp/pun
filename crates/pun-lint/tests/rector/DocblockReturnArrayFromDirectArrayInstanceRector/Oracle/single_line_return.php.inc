<?php

class SingleLineReturn
{
    /** @return array */
    public function run(): array
    {
        return [[1], [2]];
    }
}
-----
<?php

class SingleLineReturn
{
    /** @return array<int, int[]> */
    public function run(): array
    {
        return [[1], [2]];
    }
}
