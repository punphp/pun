<?php

class BlockCommentAndDoc
{
    /* plain */
    /**
     * Doc
     */
    // after
    public function run(): array
    {
        return [true, false];
    }
}
-----
<?php

class BlockCommentAndDoc
{
    /* plain */
    /**
     * Doc
     * @return array<int, bool>
     */
    // after
    public function run(): array
    {
        return [true, false];
    }
}
