<?php

namespace Illuminate\Filesystem;

use Aws\S3\S3Client;
use Illuminate\Support\Traits\Conditionable;
use League\Flysystem\FilesystemAdapter as FlysystemAdapter;
use League\Flysystem\FilesystemOperator;

class AwsS3V3Adapter extends FilesystemAdapter
{
    use Conditionable;

    /**
     * The AWS S3 client.
     *
     * @var \Aws\S3\S3Client
     */
    protected $client;

    /**
     * Create a new AwsS3V3FilesystemAdapter instance.
     *
     * @param  \League\Flysystem\FilesystemOperator  $driver
     * @param  \League\Flysystem\FilesystemAdapter  $adapter
     * @param  array  $config
     * @param  \Aws\S3\S3Client  $client
     */
    public function __construct(FilesystemOperator $driver, FlysystemAdapter $adapter, array $config, S3Client $client)
    {}

    /**
     * Get the URL for the file at the given path.
     *
     * @param  string  $path
     * @return string
     *
     * @throws \RuntimeException
     */
    public function url($path)
    {}

    /**
     * Determine if temporary URLs can be generated.
     *
     * @return bool
     */
    public function providesTemporaryUrls()
    {}

    /**
     * Determine if temporary upload URLs can be generated.
     *
     * @return bool
     */
    public function providesTemporaryUploadUrls()
    {}

    /**
     * Get a temporary URL for the file at the given path.
     *
     * @param  string  $path
     * @param  \DateTimeInterface  $expiration
     * @param  array  $options
     * @return string
     */
    public function temporaryUrl($path, $expiration, array $options = [])
    {}

    /**
     * Get a temporary upload URL for the file at the given path.
     *
     * @param  string  $path
     * @param  \DateTimeInterface  $expiration
     * @param  array  $options
     * @return array
     */
    public function temporaryUploadUrl($path, $expiration, array $options = [])
    {}

    /**
     * Get the underlying S3 client.
     *
     * @return \Aws\S3\S3Client
     */
    public function getClient()
    {}
}
