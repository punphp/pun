<?php

namespace Illuminate\Filesystem;

use League\Flysystem\Config;
use League\Flysystem\FileAttributes;
use League\Flysystem\FilesystemAdapter;
use League\Flysystem\FilesystemException;
use League\Flysystem\FilesystemOperator;
use League\Flysystem\UnableToCopyFile;
use League\Flysystem\UnableToMoveFile;
use League\Flysystem\UnableToReadFile;

class ReadThroughFilesystemAdapter implements FilesystemAdapter
{
    /**
     * Create a new read-through filesystem adapter instance.
     */
    public function __construct(
        protected FilesystemOperator $primary,
        protected FilesystemOperator $fallback,
        protected bool $throwOnPromotionFailure = false,
        protected bool $copy = true,
    ) {}

    /**
     * Determine if a file exists.
     */
    public function fileExists(string $path): bool
    {}

    /**
     * Determine if a directory exists.
     */
    public function directoryExists(string $path): bool
    {}

    /**
     * Write a file to the primary filesystem.
     */
    public function write(string $path, string $contents, Config $config): void
    {}

    /**
     * Write a file stream to the primary filesystem.
     */
    public function writeStream(string $path, $contents, Config $config): void
    {}

    /**
     * Read a file, promoting it from the fallback filesystem when necessary.
     */
    public function read(string $path): string
    {}

    /**
     * Read a file stream, promoting it from the fallback filesystem when necessary.
     */
    public function readStream(string $path)
    {}

    /**
     * Delete a file from both filesystems.
     */
    public function delete(string $path): void
    {}

    /**
     * Delete a directory from both filesystems.
     */
    public function deleteDirectory(string $path): void
    {}

    /**
     * Create a directory on the primary filesystem.
     */
    public function createDirectory(string $path, Config $config): void
    {}

    /**
     * Set a file's visibility on the primary filesystem.
     */
    public function setVisibility(string $path, string $visibility): void
    {}

    /**
     * Retrieve a file's visibility.
     */
    public function visibility(string $path): FileAttributes
    {}

    /**
     * Retrieve a file's MIME type.
     */
    public function mimeType(string $path): FileAttributes
    {}

    /**
     * Retrieve a file's last modified time.
     */
    public function lastModified(string $path): FileAttributes
    {}

    /**
     * Retrieve a file's size.
     */
    public function fileSize(string $path): FileAttributes
    {}

    /**
     * List the contents of the primary filesystem.
     */
    public function listContents(string $path, bool $deep): iterable
    {}

    /**
     * Move a file on the primary filesystem and remove its fallback source.
     */
    public function move(string $source, string $destination, Config $config): void
    {}

    /**
     * Copy a file on the primary filesystem, reading it from the fallback filesystem when necessary.
     */
    public function copy(string $source, string $destination, Config $config): void
    {}

    /**
     * Move a file to its destination on the primary filesystem.
     */
    protected function moveOnPrimary(string $source, string $destination, Config $config): void
    {}

    /**
     * Copy a file from the fallback filesystem to the primary filesystem.
     */
    protected function copyFromFallback(string $source, string $destination, Config $config): void
    {}

    /**
     * Get the filesystem that contains the given path.
     */
    protected function readerFor(string $path): FilesystemOperator
    {}

    /**
     * Handle an exception encountered while promoting a file.
     */
    protected function handlePromotionFailure(string $path, FilesystemException $exception): void
    {}
}
