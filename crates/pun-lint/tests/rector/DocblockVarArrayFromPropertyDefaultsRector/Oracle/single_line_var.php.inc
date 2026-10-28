<?php

final class SingleLineVar
{
    /** @var array */
    private array $ids = [1, 2];
}
-----
<?php

final class SingleLineVar
{
    /** @var int[] */
    private array $ids = [1, 2];
}
