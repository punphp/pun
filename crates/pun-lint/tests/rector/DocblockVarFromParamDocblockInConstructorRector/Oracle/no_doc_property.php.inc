<?php

final class NoDocProperty
{
    private array $ids;

    /**
     * @param int[] $ids the ids
     */
    public function __construct(array $ids)
    {
        $this->ids = $ids;
    }
}
-----
<?php

final class NoDocProperty
{
    /**
     * @var int[]
     */
    private array $ids;

    /**
     * @param int[] $ids the ids
     */
    public function __construct(array $ids)
    {
        $this->ids = $ids;
    }
}
