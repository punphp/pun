<?php

declare(strict_types=1);

namespace League\Flysystem;

use DateTimeInterface;
use Generator;
use League\Flysystem\UrlGeneration\PrefixPublicUrlGenerator;
use League\Flysystem\UrlGeneration\PublicUrlGenerator;
use League\Flysystem\UrlGeneration\ShardedPrefixPublicUrlGenerator;
use League\Flysystem\UrlGeneration\TemporaryUrlGenerator;
use Throwable;

use function array_key_exists;
use function is_array;

class Filesystem implements FilesystemOperator
{
    use CalculateChecksumFromStream;

    private Config $config;
    private PathNormalizer $pathNormalizer;

    public function __construct(
        private FilesystemAdapter $adapter,
        array $config = [],
        ?PathNormalizer $pathNormalizer = null,
        private ?PublicUrlGenerator $publicUrlGenerator = null,
        private ?TemporaryUrlGenerator $temporaryUrlGenerator = null,
    ) {}

    public function fileExists(string $location): bool
    {}

    public function directoryExists(string $location): bool
    {}

    public function has(string $location): bool
    {}

    public function write(string $location, string $contents, array $config = []): void
    {}

    public function writeStream(string $location, $contents, array $config = []): void
    {}

    public function read(string $location): string
    {}

    public function readStream(string $location)
    {}

    public function delete(string $location): void
    {}

    public function deleteDirectory(string $location): void
    {}

    public function createDirectory(string $location, array $config = []): void
    {}

    public function listContents(string $location, bool $deep = self::LIST_SHALLOW): DirectoryListing
    {}

    private function pipeListing(string $location, bool $deep, iterable $listing): Generator
    { yield; }

    public function move(string $source, string $destination, array $config = []): void
    {}

    public function copy(string $source, string $destination, array $config = []): void
    {}

    public function lastModified(string $path): int
    {}

    public function fileSize(string $path): int
    {}

    public function mimeType(string $path): string
    {}


    public function setVisibility(string $path, string $visibility): void
    {}

    public function visibility(string $path): string
    {}

    public function publicUrl(string $path, array $config = []): string
    {}

    public function temporaryUrl(string $path, DateTimeInterface $expiresAt, array $config = []): string
    {}

    public function checksum(string $path, array $config = []): string
    {}

    private function resolvePublicUrlGenerator(): ?PublicUrlGenerator
    {}

    /**
     * @param mixed $contents
     */
    private function assertIsResource($contents): void
    {}

    /**
     * @param resource $resource
     */
    private function rewindStream($resource): void
    {}

    private function resolveConfigForMoveAndCopy(array $config): Config
    {}
}
