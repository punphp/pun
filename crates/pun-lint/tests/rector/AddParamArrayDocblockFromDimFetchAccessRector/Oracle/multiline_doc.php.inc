<?php

final class MultilineDoc
{
    /**
     * @param array<
     *     int
     * > $other
     */
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
-----
<?php

final class MultilineDoc
{
    /**
     * @param array<
     *     int
     * > $other
     * @param array<string, mixed> $data
     */
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
