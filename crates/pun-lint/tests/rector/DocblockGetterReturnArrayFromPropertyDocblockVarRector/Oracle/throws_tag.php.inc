<?php

final class ThrowsTag
{
    /** @var array<string, int> */
    private array $counts = [];

    /**
     * Gives the counts.
     *
     * @throws \RuntimeException
     */
    public function getCounts(): array
    {
        return $this->counts;
    }
}
-----
<?php

final class ThrowsTag
{
    /** @var array<string, int> */
    private array $counts = [];

    /**
     * Gives the counts.
     *
     * @throws \RuntimeException
     * @return array<string, int>
     */
    public function getCounts(): array
    {
        return $this->counts;
    }
}
