<?php

final class StringsExistingDoc
{
    /**
     * Gives names.
     *
     * @deprecated
     */
    public function names(): array
    {
        $names = [];
        $names[] = 'a';
        $names[] = 'b';

        return $names;
    }
}
-----
<?php

final class StringsExistingDoc
{
    /**
     * Gives names.
     *
     * @deprecated
     * @return string[]
     */
    public function names(): array
    {
        $names = [];
        $names[] = 'a';
        $names[] = 'b';

        return $names;
    }
}
