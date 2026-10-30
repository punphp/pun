<?php

class CloseOnTextLine
{
    /**
     * Text */
    public function run(): array
    {
        return ['a' => 1];
    }
}
-----
<?php

class CloseOnTextLine
{
    /**
     * Text
     * @return array<string, int> */
    public function run(): array
    {
        return ['a' => 1];
    }
}
