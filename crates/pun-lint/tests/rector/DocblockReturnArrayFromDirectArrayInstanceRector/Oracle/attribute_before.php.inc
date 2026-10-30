<?php

class AttributeBefore
{
    /**
     * Text
     */
    #[\Deprecated]
    public function run(): array
    {
        return ['a' => 1];
    }
}
-----
<?php

class AttributeBefore
{
    /**
     * Text
     * @return array<string, int>
     */
    #[\Deprecated]
    public function run(): array
    {
        return ['a' => 1];
    }
}
