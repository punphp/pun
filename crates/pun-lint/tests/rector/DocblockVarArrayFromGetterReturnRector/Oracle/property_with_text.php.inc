<?php

final class PropertyWithText
{
    /**
     * The ids.
     */
    private array $ids = [];

    /**
     * @return int[]
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
-----
<?php

final class PropertyWithText
{
    /**
     * The ids.
     * @var int[]
     */
    private array $ids = [];

    /**
     * @return int[]
     */
    public function getIds(): array
    {
        return $this->ids;
    }
}
