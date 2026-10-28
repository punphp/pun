<?php

final class SomeClass
{
    /**
     * @return array<
     *     string
     * >
     */
    public function run(string $contents): array
    {
        return json_decode($contents, true);
    }
}
