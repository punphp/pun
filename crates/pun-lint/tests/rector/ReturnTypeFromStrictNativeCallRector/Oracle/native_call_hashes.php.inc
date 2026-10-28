<?php

namespace App;

final class NativeHashes
{
    /**
     * @param array<string, 'a'|null> $a
     */
    public function constantItems(array $a, bool $b)
    {
        if ($b) {
            return array_slice($a, 0);
        }

        return count($a);
    }

    /**
     * @param array<string, int|string> $a
     */
    public function generalItems(array $a, bool $b)
    {
        if ($b) {
            return array_slice($a, 0);
        }

        return count($a);
    }

    public function split(string $s)
    {
        return preg_split('/,/', $s);
    }
}
-----
<?php

namespace App;

final class NativeHashes
{
    /**
     * @param array<string, 'a'|null> $a
     */
    public function constantItems(array $a, bool $b): array|int
    {
        if ($b) {
            return array_slice($a, 0);
        }

        return count($a);
    }

    /**
     * @param array<string, int|string> $a
     */
    public function generalItems(array $a, bool $b): array|int
    {
        if ($b) {
            return array_slice($a, 0);
        }

        return count($a);
    }

    public function split(string $s)
    {
        return preg_split('/,/', $s);
    }
}
