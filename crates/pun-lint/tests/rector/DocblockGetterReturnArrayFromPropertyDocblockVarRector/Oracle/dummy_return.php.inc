<?php

final class DummyReturn
{
    /**
     * @var int[]
     */
    private array $ids = [];

    /**
     * @return array the ids
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
-----
<?php

final class DummyReturn
{
    /**
     * @var int[]
     */
    private array $ids = [];

    /**
     * @return int[] the ids
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
