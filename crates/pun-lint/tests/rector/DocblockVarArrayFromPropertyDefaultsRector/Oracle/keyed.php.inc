<?php

final class Keyed
{
    private array $map = ['a' => 1, 'b' => 2];

    protected array $nested = [['x' => 1.5]];
}
-----
<?php

final class Keyed
{
    /**
     * @var array<string, int>
     */
    private array $map = ['a' => 1, 'b' => 2];

    protected array $nested = [['x' => 1.5]];
}
