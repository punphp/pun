<?php

class NoSpaceAfterStar
{
    /**
     *Text
     *@param int $a
     */
    public function run($a): array
    {
        return ['a' => 1];
    }
}
-----
<?php

class NoSpaceAfterStar
{
    /**
     *Text
     *@param int $a
     * @return array<string, int>
     */
    public function run($a): array
    {
        return ['a' => 1];
    }
}
