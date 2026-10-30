<?php

final class GenericReturn
{
    private array $map = [];

    /**
     * @return array<string, list<int>>
     */
    public function getMap(): array
    {
        return $this->map;
    }
}
-----
<?php

final class GenericReturn
{
    /**
     * @var array<string, list<int>>
     */
    private array $map = [];

    /**
     * @return array<string, list<int>>
     */
    public function getMap(): array
    {
        return $this->map;
    }
}
