<?php

final class TwoParams
{
    /**
     * @param string $prefix
     */
    public function process(string $prefix, array $data): void
    {
        echo $prefix . $data['key'];
    }
}
-----
<?php

final class TwoParams
{
    /**
     * @param string $prefix
     * @param array<string, mixed> $data
     */
    public function process(string $prefix, array $data): void
    {
        echo $prefix . $data['key'];
    }
}
