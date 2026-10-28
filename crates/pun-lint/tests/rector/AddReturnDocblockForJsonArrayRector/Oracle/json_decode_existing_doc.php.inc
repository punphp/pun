<?php

final class JsonDecodeExistingDoc
{
    /**
     * Decode it.
     *
     * @throws \JsonException
     */
    public function provide(string $contents): array
    {
        return json_decode($contents, true, 512, JSON_THROW_ON_ERROR);
    }
}
-----
<?php

final class JsonDecodeExistingDoc
{
    /**
     * Decode it.
     *
     * @throws \JsonException
     * @return array<string, mixed>
     */
    public function provide(string $contents): array
    {
        return json_decode($contents, true, 512, JSON_THROW_ON_ERROR);
    }
}
