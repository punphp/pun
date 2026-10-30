<?php

class ReturnWithDescription
{
    /**
     * @return array   the items
     * @throws Exception
     */
    public function run(): array
    {
        return ['x' => [1, 'a']];
    }
}
-----
<?php

class ReturnWithDescription
{
    /**
     * @return array<string, array<int, int|string>> the items
     * @throws Exception
     */
    public function run(): array
    {
        return ['x' => [1, 'a']];
    }
}
