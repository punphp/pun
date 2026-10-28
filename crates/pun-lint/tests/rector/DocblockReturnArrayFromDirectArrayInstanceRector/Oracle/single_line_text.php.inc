<?php

class SingleLineText
{
    /** Some text */
    public function run(): array
    {
        return [1, 2];
    }
}
-----
<?php

class SingleLineText
{
    /** Some text
     * @return array<int, int> */
    public function run(): array
    {
        return [1, 2];
    }
}
