<?php

final class SomeClass
{
    /**
     * @return mixed
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
     * @return array<string, mixed>
     */
    public function run(string $contents): array
    {
        return json_decode($contents, true);
    }
}
