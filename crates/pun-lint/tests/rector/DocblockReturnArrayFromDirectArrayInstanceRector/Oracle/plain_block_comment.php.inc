<?php

class PlainBlockComment
{
    /* not a doc */
    private function run(): array
    {
        return ['k' => 'v'];
    }
}
-----
<?php

class PlainBlockComment
{
    /* not a doc */
    /**
     * @return array<string, string>
     */
    private function run(): array
    {
        return ['k' => 'v'];
    }
}
