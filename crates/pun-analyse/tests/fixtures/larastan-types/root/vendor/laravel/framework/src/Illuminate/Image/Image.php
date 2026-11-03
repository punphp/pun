<?php

namespace Illuminate\Image;

use Closure;
use finfo;
use Illuminate\Container\Container;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Contracts\Image\Driver;
use Illuminate\Contracts\Image\Transformation;
use Illuminate\Contracts\Support\Responsable;
use Illuminate\Http\Response;
use Illuminate\Http\UploadedFile;
use Illuminate\Image\Transformations\Blur;
use Illuminate\Image\Transformations\Contain;
use Illuminate\Image\Transformations\Cover;
use Illuminate\Image\Transformations\Crop;
use Illuminate\Image\Transformations\FlipHorizontally;
use Illuminate\Image\Transformations\FlipVertically;
use Illuminate\Image\Transformations\Grayscale;
use Illuminate\Image\Transformations\Orient;
use Illuminate\Image\Transformations\Resize;
use Illuminate\Image\Transformations\Rotate;
use Illuminate\Image\Transformations\Scale;
use Illuminate\Image\Transformations\Sharpen;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use Stringable;
use Throwable;

class Image implements Responsable, Stringable
{
    use Conditionable, Macroable;

    /**
     * The image processing pipeline.
     */
    protected ImagePipeline $pipeline;

    /**
     * The driver override.
     */
    protected ?string $driver = null;

    /**
     * The image contents, or a lazy loader that resolves them.
     */
    protected Closure|string $contents;

    /**
     * Whether the image has been processed.
     */
    protected bool $processed = false;

    /**
     * The cached hash name.
     */
    protected ?string $hashName = null;

    /**
     * Create a new image instance.
     */
    public function __construct(
        Closure|string $contents,
        protected ?UploadedFile $file = null,
    ) {}

    /**
     * Wrap a lazy contents loader so it is invoked at most once, even across clones.
     */
    protected function resolveOnce(Closure $loader): Closure
    {}

    /**
     * Set the cover dimensions.
     *
     * @param  int<1, max>  $width
     * @param  int<1, max>  $height
     */
    public function cover(int $width, int $height): static
    {}

    /**
     * Set the contain dimensions.
     *
     * @param  int<1, max>  $width
     * @param  int<1, max>  $height
     */
    public function contain(int $width, int $height, ?string $background = null): static
    {}

    /**
     * Crop the image to the given dimensions and position.
     *
     * @param  int<1, max>  $width
     * @param  int<1, max>  $height
     */
    public function crop(int $width, int $height, int $x = 0, int $y = 0): static
    {}

    /**
     * Resize the image to the given dimensions.
     *
     * @param  int<1, max>|null  $width
     * @param  int<1, max>|null  $height
     */
    public function resize(?int $width = null, ?int $height = null): static
    {}

    /**
     * Rotate the image clockwise by the given angle.
     */
    public function rotate(float $angle, ?string $background = null): static
    {}

    /**
     * Set the scale dimensions.
     *
     * @param  int<1, max>|null  $width
     * @param  int<1, max>|null  $height
     */
    public function scale(?int $width = null, ?int $height = null): static
    {}

    /**
     * Auto-orient the image based on EXIF data.
     */
    public function orient(): static
    {}

    /**
     * Apply a blur effect.
     *
     * @param  int<0, 100>  $amount
     */
    public function blur(int $amount = 5): static
    {}

    /**
     * Convert the image to grayscale.
     */
    public function grayscale(): static
    {}

    /**
     * Sharpen the image.
     *
     * @param  int<0, 100>  $amount
     */
    public function sharpen(int $amount = 10): static
    {}

    /**
     * Flip the image vertically.
     */
    public function flipVertically(): static
    {}

    /**
     * Flip the image horizontally.
     */
    public function flipHorizontally(): static
    {}

    /**
     * Flip the image vertically.
     */
    public function flip(): static
    {}

    /**
     * Flip the image horizontally.
     */
    public function flop(): static
    {}

    /**
     * Add a transformation to the image pipeline.
     */
    public function transform(Transformation $transformation): static
    {}

    /**
     * Set the optimization options.
     *
     * @throws ImageException
     */
    public function optimize(string $format = 'webp', int $quality = ImageOutputOptions::DEFAULT_QUALITY): static
    {}

    /**
     * Set the output quality.
     *
     * @param  int<1, 100>  $quality
     */
    public function quality(int $quality): static
    {}

    /**
     * Convert the image to WebP format.
     */
    public function toWebp(): static
    {}

    /**
     * Convert the image to JPEG format.
     */
    public function toJpg(): static
    {}

    /**
     * Convert the image to JPEG format.
     */
    public function toJpeg(): static
    {}

    /**
     * Convert the image to PNG format.
     */
    public function toPng(): static
    {}

    /**
     * Convert the image to GIF format.
     */
    public function toGif(): static
    {}

    /**
     * Convert the image to AVIF format.
     */
    public function toAvif(): static
    {}

    /**
     * Convert the image to HEIC format.
     */
    public function toHeic(): static
    {}

    /**
     * Convert the image to JPEG XL format.
     */
    public function toJxl(): static
    {}

    /**
     * Convert the image to BMP format.
     */
    public function toBmp(): static
    {}

    /**
     * Set the output format.
     *
     * @throws ImageException
     */
    public function toFormat(string $format): static
    {}

    /**
     * Store the processed image on a filesystem disk.
     *
     * @param  array<string, mixed>  $options
     */
    public function store(string $path = '', ?string $disk = null, array $options = []): string|false
    {}

    /**
     * Store the processed image on a filesystem disk with public visibility.
     *
     * @param  array<string, mixed>  $options
     */
    public function storePublicly(string $path = '', ?string $disk = null, array $options = []): string|false
    {}

    /**
     * Store the processed image on a filesystem disk with a given name.
     *
     * @param  array<string, mixed>  $options
     */
    public function storeAs(string $path, ?string $name = null, ?string $disk = null, array $options = []): string|false
    {}

    /**
     * Store the processed image on a filesystem disk with public visibility and a given name.
     *
     * @param  array<string, mixed>  $options
     */
    public function storePubliclyAs(string $path, ?string $name = null, ?string $disk = null, array $options = []): string|false
    {}

    /**
     * Get a hashed filename with the correct extension.
     */
    public function hashName(string $path = ''): string
    {}

    /**
     * Process the image and return the raw bytes.
     */
    public function toBytes(): string
    {}

    /**
     * Process the image and return as a base64 encoded string.
     */
    public function toBase64(): string
    {}

    /**
     * Process the image and return as a data URI.
     */
    public function toDataUri(): string
    {}

    /**
     * Get the file extension based on the MIME type.
     */
    public function extension(): string
    {}

    /**
     * Get the MIME type of the processed image.
     */
    public function mimeType(): string
    {}

    /**
     * Get the dimensions of the processed image.
     *
     * @return array{0: int, 1: int}
     */
    public function dimensions(): array
    {}

    /**
     * Get the width of the processed image.
     */
    public function width(): int
    {}

    /**
     * Get the height of the processed image.
     */
    public function height(): int
    {}

    /**
     * Get the dominant (average) color of the image as a hex string.
     */
    public function dominantColor(): string
    {}

    /**
     * Set the driver to use for processing.
     */
    public function using(string $driver): static
    {}

    /**
     * Use the GD driver for processing.
     */
    public function usingGd(): static
    {}

    /**
     * Use the Imagick driver for processing.
     */
    public function usingImagick(): static
    {}

    /**
     * Resolve the image processing driver.
     */
    protected function resolveDriver(): Driver
    {}

    /**
     * Get the underlying uploaded file instance.
     */
    public function file(): ?UploadedFile
    {}

    /**
     * Create an immutable clone with copied options.
     */
    protected function newClone(): static
    {}

    /**
     * Create an immutable clone with updated output options.
     */
    protected function withOutput(Closure $callback): static
    {}

    /**
     * Create an immutable clone with the given callback applied.
     */
    protected function withClone(Closure $callback): static
    {}

    /**
     * Create an HTTP response that represents the image.
     *
     * @param  \Illuminate\Http\Request  $request
     */
    public function toResponse($request): Response
    {}

    /**
     * Prevent serialization of the image.
     *
     * @throws ImageException
     */
    public function __serialize(): never
    {}

    /**
     * Get the string representation of the image.
     */
    public function toString(): string
    {}

    /**
     * Get the string representation of the image.
     */
    public function __toString(): string
    {}
}
