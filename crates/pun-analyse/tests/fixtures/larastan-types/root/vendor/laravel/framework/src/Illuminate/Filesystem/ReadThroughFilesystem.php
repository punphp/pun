<?php

namespace Illuminate\Filesystem;

use League\Flysystem\FilesystemAdapter as FlysystemAdapter;
use League\Flysystem\FilesystemOperator;

class ReadThroughFilesystem extends FilesystemAdapter
{
    /**
     * Create a new read-through filesystem instance.
     */
    public function __construct(
        FilesystemOperator $driver,
        FlysystemAdapter $adapter,
        array $config,
        protected FilesystemAdapter $primary,
        protected FilesystemAdapter $fallback,
    ) {}

    /**
     * Get the URL for the file at the given path.
     */
    public function url($path)
    {}

    /**
     * Get a temporary URL for the file at the given path.
     */
    public function temporaryUrl($path, $expiration, array $options = [])
    {}

    /**
     * Determine if temporary upload URLs can be generated.
     */
    public function providesTemporaryUploadUrls()
    {}

    /**
     * Get a temporary upload URL for the file at the given path.
     */
    public function temporaryUploadUrl($path, $expiration, array $options = [])
    {}

    /**
     * Determine if temporary URLs can be generated.
     */
    public function providesTemporaryUrls()
    {}

    /**
     * Get the filesystem that contains the given path.
     */
    protected function readerFor($path)
    {}
}
