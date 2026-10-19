<?php

namespace App;

final class R
{
    public function keys($a)
    {
        if (preg_match('/x/', $a, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    public function first($a)
    {
        return explode('|', $a, 3)[0];
    }

    public function parts($a)
    {
        $parts = explode('|', $a);

        return $parts;
    }
}
-----
<?php

namespace App;

final class R
{
    public function keys($a): array
    {
        if (preg_match('/x/', $a, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    public function first($a)
    {
        return explode('|', $a, 3)[0];
    }

    public function parts($a)
    {
        $parts = explode('|', $a);

        return $parts;
    }
}
