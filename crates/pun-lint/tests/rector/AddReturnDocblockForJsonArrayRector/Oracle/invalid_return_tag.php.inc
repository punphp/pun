<?php

final class SomeClass
{
    /**
     * @return <
     */
    public function run(string $contents): array
    {
        return json_decode($contents, true);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @return <
     * @return array<string, mixed>
     */
    public function run(string $contents): array
    {
        return json_decode($contents, true);
    }
}
