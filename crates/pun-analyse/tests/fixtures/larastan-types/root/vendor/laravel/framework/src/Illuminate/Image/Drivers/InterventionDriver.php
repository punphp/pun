<?php

namespace Illuminate\Image\Drivers;

use finfo;
use Illuminate\Contracts\Image\Driver;
use Illuminate\Image\ImageException;
use Illuminate\Image\ImageOutputOptions;
use Illuminate\Image\ImagePipeline;
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
use Intervention\Image\Direction;
use Intervention\Image\Encoders\AvifEncoder;
use Intervention\Image\Encoders\BmpEncoder;
use Intervention\Image\Encoders\GifEncoder;
use Intervention\Image\Encoders\HeicEncoder;
use Intervention\Image\Encoders\JpegEncoder;
use Intervention\Image\Encoders\JxlEncoder;
use Intervention\Image\Encoders\MediaTypeEncoder;
use Intervention\Image\Encoders\PngEncoder;
use Intervention\Image\Encoders\WebpEncoder;
use Intervention\Image\ImageManager;
use Intervention\Image\Interfaces\ImageInterface;

abstract class InterventionDriver implements Driver
{
    /**
     * The registered transformation handlers.
     *
     * @var array<class-string<\Illuminate\Contracts\Image\Transformation>, callable>
     */
    protected array $transformationHandlers = [];

    /**
     * The Intervention image manager instance.
     */
    protected ImageManager $manager;

    /**
     * Create a new Intervention driver instance.
     */
    public function __construct()
    {}

    /**
     * Create the underlying Intervention image manager.
     */
    abstract protected function createManager(): ImageManager;

    /**
     * Ensure Intervention Image is installed.
     *
     * @throws ImageException
     */
    public function ensureRequirementsAreMet(): void
    {}

    /**
     * Process the given image contents with the specified pipeline.
     */
    public function process(string $contents, ImagePipeline $pipeline): string
    {}

    /**
     * Get the dimensions of the given image contents.
     *
     * @return array{0: int, 1: int}
     */
    public function dimensions(string $contents): array
    {}

    /**
     * Resolve a background color, expanding the "dominant" sentinel when needed.
     */
    protected function resolveBackground(ImageInterface $image, ?string $background): ?string
    {}

    /**
     * Get the dominant (average) color of the image as a hex string.
     */
    public function dominantColor(string $contents): string
    {}

    /**
     * Sample the dominant color by resizing the image to a single pixel.
     */
    protected function dominantColorFrom(ImageInterface $image): string
    {}

    /**
     * Register a transformation handler.
     *
     * @param  class-string<\Illuminate\Contracts\Image\Transformation>  $transformation
     */
    public function transformUsing(string $transformation, callable $callback): static
    {}

    /**
     * Get the handler for the given transformation.
     */
    protected function transformationHandlerFor(object $transformation): ?callable
    {}
}
