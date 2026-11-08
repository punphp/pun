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
     * @return array<string, int|null>
     */
    public function run(): array
    {
        return ['a' => null, 'b' => 1];
    }
}
