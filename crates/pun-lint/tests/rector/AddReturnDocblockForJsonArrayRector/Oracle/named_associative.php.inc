<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        return json_decode($contents, associative: true);
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
        return json_decode($contents, associative: true);
    }
}
