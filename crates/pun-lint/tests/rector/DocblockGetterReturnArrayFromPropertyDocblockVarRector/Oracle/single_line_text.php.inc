<?php

final class SingleLineText
{
    /**
     * @var string[]
     */
    private array $names = [];

    /** Gives the names. */
    public function getNames(): array
    {
        return $this->names;
    }
}
-----
<?php

final class SingleLineText
{
    /**
     * @var string[]
     */
    private array $names = [];

    /**
     * Gives the names.
     * @return string[]
     */
    public function getNames(): array
    {
        return $this->names;
    }
}
