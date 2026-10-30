<?php

final class StringsWithText
{
    /**
     * The names.
     */
    public array $names = ['a', 'b'];
}
-----
<?php

final class StringsWithText
{
    /**
     * The names.
     * @var string[]
     */
    public array $names = ['a', 'b'];
}
