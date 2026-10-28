<?php

class GenericTagLast
{
    /**
     * @deprecated use other
     */
    public function run(): array
    {
        return ['a'];
    }
}
-----
<?php

class GenericTagLast
{
    /**
     * @deprecated use other
     * @return array<int, string>
     */
    public function run(): array
    {
        return ['a'];
    }
}
