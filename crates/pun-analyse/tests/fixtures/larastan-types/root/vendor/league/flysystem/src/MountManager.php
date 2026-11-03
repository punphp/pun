<?php

declare(strict_types=1);

namespace League\Flysystem;

use DateTimeInterface;
use Throwable;

use function compact;
use function method_exists;
use function sprintf;

class MountManager implements FilesystemOperator
{
    /**
     * @var array<string, FilesystemOperator>
     */
    private $filesystems = [];

    /**
     * @var Config
     */
    private $config;

    /**
     * MountManager constructor.
     *
     * @param array<string,FilesystemOperator> $filesystems
     */
    public function __construct(array $filesystems = [], array $config = [])
    {}

    /**
     * It is not recommended to mount filesystems after creation because interacting
     * with the Mount Manager becomes unpredictable. Use this as an escape hatch.
     */
    public function dangerouslyMountFilesystems(string $key, FilesystemOperator $filesystem): void
    {}

    /**
     * @param array<string,FilesystemOperator> $filesystems
     */
    public function extend(array $filesystems, array $config = []): MountManager
    {}

    public function fileExists(string $location): bool
    {}

    public function has(string $location): bool
    {}

    public function directoryExists(string $location): bool
    {}

    public function read(string $location): string
    {}

    public function readStream(string $location)
    {}

    public function listContents(string $location, bool $deep = self::LIST_SHALLOW): DirectoryListing
    {}

    public function lastModified(string $location): int
    {}

    public function fileSize(string $location): int
    {}

    public function mimeType(string $location): string
    {}

    public function visibility(string $path): string
    {}

    public function write(string $location, string $contents, array $config = []): void
    {}

    public function writeStream(string $location, $contents, array $config = []): void
    {}

    public function setVisibility(string $path, string $visibility): void
    {}

    public function delete(string $location): void
    {}

    public function deleteDirectory(string $location): void
    {}

    public function createDirectory(string $location, array $config = []): void
    {}

    public function move(string $source, string $destination, array $config = []): void
    {}

    public function copy(string $source, string $destination, array $config = []): void
    {}

    public function publicUrl(string $path, array $config = []): string
    {}

    public function temporaryUrl(string $path, DateTimeInterface $expiresAt, array $config = []): string
    {}

    public function checksum(string $path, array $config = []): string
    {}

    private function mountFilesystems(array $filesystems): void
    {}

    private function guardAgainstInvalidMount(mixed $key, mixed $filesystem): void
    {}

    private function mountFilesystem(string $key, FilesystemOperator $filesystem): void
    {}

    /**
     * @param string $path
     *
     * @return array{0:FilesystemOperator, 1:string, 2:string}
     */
    private function determineFilesystemAndPath(string $path): array
    {}

    private function copyInSameFilesystem(
        FilesystemOperator $sourceFilesystem,
        string $sourcePath,
        string $destinationPath,
        string $source,
        string $destination,
        array $config,
    ): void {}

    private function copyAcrossFilesystem(
        FilesystemOperator $sourceFilesystem,
        string $sourcePath,
        FilesystemOperator $destinationFilesystem,
        string $destinationPath,
        string $source,
        string $destination,
        array $config,
    ): void {}

    private function moveInTheSameFilesystem(
        FilesystemOperator $sourceFilesystem,
        string $sourcePath,
        string $destinationPath,
        string $source,
        string $destination,
        array $config,
    ): void {}

    private function moveAcrossFilesystems(string $source, string $destination, array $config = []): void
    {}
}
