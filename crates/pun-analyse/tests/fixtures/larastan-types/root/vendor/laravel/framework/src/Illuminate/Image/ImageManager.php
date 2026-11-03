<?php

namespace Illuminate\Image;

use BackedEnum;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Contracts\Image\Driver;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Http\Client\Factory as HttpFactory;
use Illuminate\Http\UploadedFile;
use Illuminate\Image\Drivers\GdDriver;
use Illuminate\Image\Drivers\ImagickDriver;
use Illuminate\Support\Manager;
use InvalidArgumentException;

use function Illuminate\Support\enum_value;

class ImageManager extends Manager
{
    /**
     * The registered transformation handlers.
     *
     * @var array<string, array<class-string<\Illuminate\Contracts\Image\Transformation>, callable>>
     */
    protected array $transformationHandlers = [];

    /**
     * Create an image instance from raw bytes.
     */
    public function fromBytes(string $contents): Image
    {}

    /**
     * Create an image instance from a stream.
     *
     * @param  resource  $stream
     */
    public function fromStream(mixed $stream): Image
    {}

    /**
     * Create an image instance from a base64 encoded string.
     */
    public function fromBase64(string $base64): Image
    {}

    /**
     * Create an image instance from a file path.
     */
    public function fromPath(string $path): Image
    {}

    /**
     * Create an image instance from a storage disk path.
     */
    public function fromStorage(string $path, BackedEnum|string|null $disk = null): Image
    {}

    /**
     * Create an image instance from an uploaded file.
     */
    public function fromUpload(UploadedFile $file): Image
    {}

    /**
     * Create an image instance from a URL.
     */
    public function fromUrl(string $url): Image
    {}

    /**
     * Create a new driver instance.
     *
     * @throws InvalidArgumentException
     */
    protected function createDriver($driver): Driver
    {}

    /**
     * Create the GD image driver.
     */
    protected function createGdDriver(): GdDriver
    {}

    /**
     * Create the Imagick image driver.
     */
    protected function createImagickDriver(): ImagickDriver
    {}

    /**
     * Register a transformation handler for the given driver.
     *
     * @param  class-string<\Illuminate\Contracts\Image\Transformation>  $transformation
     */
    public function transformUsing(string $driver, string $transformation, callable $callback): static
    {}

    /**
     * Apply registered transformation handlers to the given driver instance.
     */
    protected function applyTransformationHandlers(string $driver, Driver $instance): void
    {}

    /**
     * Get the default image driver name.
     */
    public function getDefaultDriver(): string
    {}
}
