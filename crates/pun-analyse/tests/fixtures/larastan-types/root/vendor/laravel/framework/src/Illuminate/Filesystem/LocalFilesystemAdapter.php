<?php

namespace Illuminate\Filesystem;

use Closure;
use Illuminate\Support\Traits\Conditionable;
use RuntimeException;

class LocalFilesystemAdapter extends FilesystemAdapter
{
    use Conditionable;

    /**
     * The name of the filesystem disk.
     *
     * @var string
     */
    protected $disk;

    /**
     * Indicates if signed URLs should serve corresponding files.
     *
     * @var bool
     */
    protected $shouldServeSignedUrls = false;

    /**
     * The Closure that should be used to resolve the URL generator.
     *
     * @var \Closure|null
     */
    protected $urlGeneratorResolver;

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
     *
     * @throws \RuntimeException
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
     *
     * @throws \RuntimeException
     */
    public function temporaryUploadUrl($path, $expiration, array $options = [])
    {}

    /**
     * Specify the name of the disk the adapter is managing.
     *
     * @param  string  $disk
     * @return $this
     */
    public function diskName(string $disk)
    {}

    /**
     * Indicate that signed URLs should serve the corresponding files.
     *
     * @param  bool  $serve
     * @param  \Closure|null  $urlGeneratorResolver
     * @return $this
     */
    public function shouldServeSignedUrls(bool $serve = true, ?Closure $urlGeneratorResolver = null)
    {}
}
