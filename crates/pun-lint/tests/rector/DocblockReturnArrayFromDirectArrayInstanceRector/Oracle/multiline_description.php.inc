<?php

class MultilineDescription
{
    /**
     * @return array first line
     *   second line
     *
     * @see Other
     */
    public function run(): array
    {
        return [1];
    }
}
-----
<?php

class MultilineDescription
{
    /**
     * @return array<int, int> first line
     *   second line
     *
     * @see Other
     */
    public function run(): array
    {
        return [1];
    }
}
