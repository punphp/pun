<?php

namespace League\Flysystem\UrlGeneration;

use InvalidArgumentException;
use League\Flysystem\Config;
use League\Flysystem\PathPrefixer;

use function array_map;
use function count;
use function crc32;

final class ShardedPrefixPublicUrlGenerator implements PublicUrlGenerator
{
    /** @var PathPrefixer[] */
    private array $prefixes;
    private int $count;

    /**
     * @param string[] $prefixes
     */
    public function __construct(array $prefixes)
    {}

    public function publicUrl(string $path, Config $config): string
    {}
}
