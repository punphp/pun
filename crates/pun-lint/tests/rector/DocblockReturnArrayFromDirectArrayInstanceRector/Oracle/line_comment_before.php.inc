<?php

class LineCommentBefore
{
    // comment
    public function run(): array
    {
        return ['a' => null, 'b' => 1];
    }
}
-----
<?php

class LineCommentBefore
{
    // comment
    /**
     * @return array<string, null|int>
     */
    public function run(): array
    {
        return ['a' => null, 'b' => 1];
    }
}
