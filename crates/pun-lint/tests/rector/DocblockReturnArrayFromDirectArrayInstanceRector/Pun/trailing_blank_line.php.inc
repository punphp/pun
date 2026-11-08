<?php

class TrailingBlankLine
{
    /**
     * Text
     *
     */
    public function run(): array
    {
        return ['a'];
    }
}
-----
<?php

class TrailingBlankLine
{
    /**
     * Text
     *
     * @return array<int, string>
     */
    public function run(): array
    {
        return ['a'];
    }
}
