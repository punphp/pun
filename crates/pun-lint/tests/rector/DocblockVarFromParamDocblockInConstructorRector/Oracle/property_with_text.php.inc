<?php

final class PropertyWithText
{
    /**
     * The names.
     */
    private array $names;

    /**
     * @param array<string, string> $names
     */
    public function __construct(array $names)
    {
        $this->names = $names;
    }
}
-----
<?php

final class PropertyWithText
{
    /**
     * The names.
     * @var array<string, string>
     */
    private array $names;

    /**
     * @param array<string, string> $names
     */
    public function __construct(array $names)
    {
        $this->names = $names;
    }
}
