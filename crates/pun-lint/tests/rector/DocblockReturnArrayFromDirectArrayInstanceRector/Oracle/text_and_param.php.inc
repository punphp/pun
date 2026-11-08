<?php

class TextAndParam
{
    /**
     * Some text
     *
     * @param int $a
     */
    public function run($a): array
    {
        return ['a' => 1, 'b' => true];
    }
}
-----
<?php

class TextAndParam
{
    /**
     * Some text
     *
     * @param int $a
     * @return array<string, bool|int>
     */
    public function run($a): array
    {
        return ['a' => 1, 'b' => true];
    }
}
