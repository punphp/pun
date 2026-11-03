<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function extension_loaded, is_array, is_int, is_string;


/**
 * Basic manipulation with images. Supported types are JPEG, PNG, GIF, WEBP, AVIF and BMP.
 *
 * <code>
 * $image = Image::fromFile('nette.jpg');
 * $image->resize(150, 100);
 * $image->sharpen();
 * $image->send();
 * </code>
 *
 * @method Image affine(array<int, float|int> $affine, ?array{x: int, y: int, width: int, height: int} $clip = null)
 * @method void alphaBlending(bool $enable)
 * @method void antialias(bool $enable)
 * @method void arc(int $centerX, int $centerY, int $width, int $height, int $startAngle, int $endAngle, ImageColor $color)
 * @method int colorAllocate(int $red, int $green, int $blue)
 * @method int colorAllocateAlpha(int $red, int $green, int $blue, int $alpha)
 * @method int colorAt(int $x, int $y)
 * @method int colorClosest(int $red, int $green, int $blue)
 * @method int colorClosestAlpha(int $red, int $green, int $blue, int $alpha)
 * @method int colorClosestHWB(int $red, int $green, int $blue)
 * @method void colorDeallocate(int $color)
 * @method int colorExact(int $red, int $green, int $blue)
 * @method int colorExactAlpha(int $red, int $green, int $blue, int $alpha)
 * @method void colorMatch(Image $image2)
 * @method int colorResolve(int $red, int $green, int $blue)
 * @method int colorResolveAlpha(int $red, int $green, int $blue, int $alpha)
 * @method void colorSet(int $index, int $red, int $green, int $blue, int $alpha = 0)
 * @method array{red: int, green: int, blue: int, alpha: int} colorsForIndex(int $color)
 * @method int colorsTotal()
 * @method int colorTransparent(?int $color = null)
 * @method void convolution(array<int, array<int, float>> $matrix, float $div, float $offset)
 * @method void copy(Image $src, int $dstX, int $dstY, int $srcX, int $srcY, int $srcW, int $srcH)
 * @method void copyMerge(Image $src, int $dstX, int $dstY, int $srcX, int $srcY, int $srcW, int $srcH, int $pct)
 * @method void copyMergeGray(Image $src, int $dstX, int $dstY, int $srcX, int $srcY, int $srcW, int $srcH, int $pct)
 * @method void copyResampled(Image $src, int $dstX, int $dstY, int $srcX, int $srcY, int $dstW, int $dstH, int $srcW, int $srcH)
 * @method void copyResized(Image $src, int $dstX, int $dstY, int $srcX, int $srcY, int $dstW, int $dstH, int $srcW, int $srcH)
 * @method Image cropAuto(int $mode = 0, float $threshold = .5, ?ImageColor $color = null)
 * @method void ellipse(int $centerX, int $centerY, int $width, int $height, ImageColor $color)
 * @method void fill(int $x, int $y, ImageColor $color)
 * @method void filledArc(int $centerX, int $centerY, int $width, int $height, int $startAngle, int $endAngle, ImageColor $color, int $style)
 * @method void filledEllipse(int $centerX, int $centerY, int $width, int $height, ImageColor $color)
 * @method void filledPolygon(array<int, int> $points, ImageColor $color)
 * @method void filledRectangle(int $x1, int $y1, int $x2, int $y2, ImageColor $color)
 * @method void fillToBorder(int $x, int $y, ImageColor $borderColor, ImageColor $color)
 * @method void filter(int $filter, ...$args)
 * @method void flip(int $mode)
 * @method array<int, int> ftText(float $size, float $angle, int $x, int $y, ImageColor $color, string $fontFile, string $text, array<string, mixed> $options = [])
 * @method void gammaCorrect(float $inputgamma, float $outputgamma)
 * @method array{int, int, int, int} getClip()
 * @method int getInterpolation()
 * @method int interlace(?bool $enable = null)
 * @method bool isTrueColor()
 * @method void layerEffect(int $effect)
 * @method void line(int $x1, int $y1, int $x2, int $y2, ImageColor $color)
 * @method void openPolygon(array<int, int> $points, ImageColor $color)
 * @method void paletteCopy(Image $source)
 * @method void paletteToTrueColor()
 * @method void polygon(array<int, int> $points, ImageColor $color)
 * @method void rectangle(int $x1, int $y1, int $x2, int $y2, ImageColor $color)
 * @method mixed resolution(?int $resolutionX = null, ?int $resolutionY = null)
 * @method Image rotate(float $angle, ImageColor $backgroundColor)
 * @method void saveAlpha(bool $enable)
 * @method Image scale(int $newWidth, int $newHeight = -1, int $mode = 3)
 * @method void setBrush(Image $brush)
 * @method void setClip(int $x1, int $y1, int $x2, int $y2)
 * @method void setInterpolation(int $method = 3)
 * @method void setPixel(int $x, int $y, ImageColor $color)
 * @method void setStyle(array<int, int> $style)
 * @method void setThickness(int $thickness)
 * @method void setTile(Image $tile)
 * @method void trueColorToPalette(bool $dither, int $ncolors)
 * @method array<int, int> ttfText(float $size, float $angle, int $x, int $y, ImageColor $color, string $fontfile, string $text, array<string, mixed> $options = [])
 * @property-read positive-int $width
 * @property-read positive-int $height
 * @property-read \GdImage $imageResource
 */
class Image
{
	use Nette\SmartObject;

	/** Prevent from getting resized to a bigger size than the original */
	public const ShrinkOnly = 0b0001;

	/** Resizes to a specified width and height without keeping aspect ratio */
	public const Stretch = 0b0010;

	/** Resizes to fit into a specified width and height and preserves aspect ratio */
	public const OrSmaller = 0b0000;

	/** Resizes while bounding the smaller dimension to the specified width or height and preserves aspect ratio */
	public const OrBigger = 0b0100;

	/** Resizes to the smallest possible size to completely cover specified width and height and reserves aspect ratio */
	public const Cover = 0b1000;

	/** @deprecated use Image::ShrinkOnly */
	public const SHRINK_ONLY = self::ShrinkOnly;

	/** @deprecated use Image::Stretch */
	public const STRETCH = self::Stretch;

	/** @deprecated use Image::OrSmaller */
	public const FIT = self::OrSmaller;

	/** @deprecated use Image::OrBigger */
	public const FILL = self::OrBigger;

	/** @deprecated use Image::Cover */
	public const EXACT = self::Cover;

	/** @deprecated use Image::EmptyGIF */
	public const EMPTY_GIF = self::EmptyGIF;

	/** image types */
	public const
		JPEG = ImageType::JPEG,
		PNG = ImageType::PNG,
		GIF = ImageType::GIF,
		WEBP = ImageType::WEBP,
		AVIF = ImageType::AVIF,
		BMP = ImageType::BMP;

	public const EmptyGIF = "GIF89a\x01\x00\x01\x00\x80\x00\x00\x00\x00\x00\x00\x00\x00!\xf9\x04\x01\x00\x00\x00\x00,\x00\x00\x00\x00\x01\x00\x01\x00\x00\x02\x02D\x01\x00;";

	private const Formats = [
		ImageType::JPEG => 'jpeg', ImageType::PNG => 'png', ImageType::GIF => 'gif', ImageType::WEBP => 'webp', ImageType::AVIF => 'avif',
		ImageType::BMP => 'bmp',
	];
	private const Sentinel = "\0";

	private \GdImage $image;


	/**
	 * Returns RGB color (0..255) and transparency (0..127).
	 * @deprecated use ImageColor::rgb()
	 * @return array{red: int, green: int, blue: int, alpha: int}
	 */
	public static function rgb(int $red, int $green, int $blue, int $transparency = 0): array
	{}


	/**
	 * Reads an image from a file and returns its type in $type. If $warnings is passed, recoverable decoder
	 * warnings are returned in it instead of being raised as a PHP warning.
	 * @param-out ?string  $warnings
	 * @throws Nette\NotSupportedException if gd extension is not loaded
	 * @throws UnknownImageFileException if file not found or file type is not known
	 */
	public static function fromFile(string $file, ?int &$type = null, ?string &$warnings = self::Sentinel): static
	{}


	/**
	 * Reads an image from a string and returns its type in $type. If $warnings is passed, recoverable decoder
	 * warnings are returned in it instead of being raised as a PHP warning.
	 * @param-out ?string  $warnings
	 * @throws Nette\NotSupportedException if gd extension is not loaded
	 * @throws ImageException
	 */
	public static function fromString(string $s, ?int &$type = null, ?string &$warnings = self::Sentinel): static
	{}


	/** @param  callable-string  $func */
	private static function invokeSafe(
		string $func,
		string $arg,
		string $message,
		string $callee,
		?string &$warnings = self::Sentinel,
	): static
	{}


	/**
	 * Creates a new true color image of the given dimensions. The default color is black.
	 * @param  positive-int  $width
	 * @param  positive-int  $height
	 * @param  ImageColor|array{red: int, green: int, blue: int, alpha?: int}|null  $color
	 * @throws Nette\NotSupportedException if gd extension is not loaded
	 */
	public static function fromBlank(int $width, int $height, ImageColor|array|null $color = null): static
	{}


	/**
	 * Returns the type of image from file.
	 * @param-out ?int  $width
	 * @param-out ?int  $height
	 * @return ?ImageType::*
	 */
	public static function detectTypeFromFile(string $file, mixed &$width = null, mixed &$height = null): ?int
	{}


	/**
	 * Returns the type of image from string.
	 * @param-out ?int  $width
	 * @param-out ?int  $height
	 * @return ?ImageType::*
	 */
	public static function detectTypeFromString(string $s, mixed &$width = null, mixed &$height = null): ?int
	{}


	/**
	 * Returns the file extension for the given image type.
	 * @param  ImageType::*  $type
	 * @return value-of<self::Formats>
	 */
	public static function typeToExtension(int $type): string
	{}


	/**
	 * Returns the image type for given file extension.
	 * @return ImageType::*
	 */
	public static function extensionToType(string $extension): int
	{}


	/**
	 * Returns the mime type for the given image type.
	 * @param  ImageType::*  $type
	 */
	public static function typeToMimeType(int $type): string
	{}


	/**
	 * Checks whether the given image type is supported by the GD extension.
	 * @param  ImageType::*  $type
	 */
	public static function isTypeSupported(int $type): bool
	{}


	/**
	 * Returns list of image types supported by the GD extension.
	 * @return  ImageType::*[]
	 */
	public static function getSupportedTypes(): array
	{}


	/**
	 * Wraps GD image.
	 */
	public function __construct(\GdImage $image)
	{}


	/**
	 * Returns image width.
	 * @return positive-int
	 */
	public function getWidth(): int
	{}


	/**
	 * Returns image height.
	 * @return positive-int
	 */
	public function getHeight(): int
	{}


	/**
	 * Sets image resource.
	 */
	protected function setImageResource(\GdImage $image): static
	{}


	/**
	 * Returns image GD resource.
	 */
	public function getImageResource(): \GdImage
	{}


	/**
	 * Scales an image. Width and height accept pixels or percent.
	 * @param  int-mask-of<self::OrSmaller|self::OrBigger|self::Stretch|self::Cover|self::ShrinkOnly>  $mode
	 */
	public function resize(int|string|null $width, int|string|null $height, int $mode = self::OrSmaller): static
	{}


	/**
	 * Calculates dimensions of resized image. Width and height accept pixels or percent.
	 * @param  int-mask-of<self::OrSmaller|self::OrBigger|self::Stretch|self::Cover|self::ShrinkOnly>  $mode
	 * @return array{int<1, max>, int<1, max>}
	 */
	public static function calculateSize(
		int $srcWidth,
		int $srcHeight,
		int|string|null $newWidth,
		int|string|null $newHeight,
		int $mode = self::OrSmaller,
	): array
	{}


	/**
	 * Crops image. Arguments accepts pixels or percent.
	 */
	public function crop(int|string $left, int|string $top, int|string $width, int|string $height): static
	{}


	/**
	 * Calculates dimensions of cutout in image. Arguments accepts pixels or percent.
	 * @return array{int, int, int, int}
	 */
	public static function calculateCutout(
		int $srcWidth,
		int $srcHeight,
		int|string $left,
		int|string $top,
		int|string $newWidth,
		int|string $newHeight,
	): array
	{}


	/**
	 * Sharpens image a little bit.
	 */
	public function sharpen(): static
	{}


	/**
	 * Puts another image into this image. Left and top accepts pixels or percent.
	 * @param  int<0, 100>  $opacity 0..100
	 */
	public function place(self $image, int|string $left = 0, int|string $top = 0, int $opacity = 100): static
	{}


	/**
	 * Calculates the bounding box for a TrueType text. Returns keys left, top, width and height.
	 * @param  array<string, mixed>  $options
	 * @return array{left: int, top: int, width: int, height: int}
	 */
	public static function calculateTextBox(
		string $text,
		string $fontFile,
		float $size,
		float $angle = 0,
		array $options = [],
	): array
	{}


	/**
	 * Draws a rectangle using top-left coordinates and dimensions instead of two corner coordinates.
	 */
	public function rectangleWH(int $x, int $y, int $width, int $height, ImageColor $color): void
	{}


	/**
	 * Draws a filled rectangle using top-left coordinates and dimensions instead of two corner coordinates.
	 */
	public function filledRectangleWH(int $x, int $y, int $width, int $height, ImageColor $color): void
	{}


	/**
	 * Saves image to the file. Quality is 0..100 for JPEG (default 85), WEBP (default 80) and AVIF (default 30), 0..9 for PNG (default 9).
	 * @param  ?ImageType::*  $type
	 * @throws ImageException
	 */
	public function save(string $file, ?int $quality = null, ?int $type = null): void
	{}


	/**
	 * Outputs image to string. Quality is 0..100 for JPEG (default 85), WEBP (default 80) and AVIF (default 30), 0..9 for PNG (default 9).
	 * @param  ImageType::*  $type
	 */
	public function toString(int $type = ImageType::JPEG, ?int $quality = null): string
	{}


	/**
	 * Outputs image to string.
	 */
	public function __toString(): string
	{}


	/**
	 * Outputs image to browser. Quality is 0..100 for JPEG (default 85), WEBP (default 80) and AVIF (default 30), 0..9 for PNG (default 9).
	 * @param  ImageType::*  $type
	 * @throws ImageException
	 */
	public function send(int $type = ImageType::JPEG, ?int $quality = null): void
	{}


	/**
	 * Outputs image to browser or file.
	 * @param  ImageType::*  $type
	 * @throws ImageException
	 */
	private function output(int $type, ?int $quality, ?string $file = null): void
	{}


	/**
	 * Call to undefined method.
	 * @param  mixed[]  $args
	 * @throws Nette\MemberAccessException
	 */
	public function __call(string $name, array $args): mixed
	{}


	public function __clone()
	{}


	/** @param-out int|float $num */
	private static function isPercent(int|string &$num): bool
	{}


	/**
	 * Prevents serialization.
	 */
	public function __serialize(): array
	{}


	/**
	 * Resolves a color to a GD color index for the current image.
	 * @param  ImageColor|array{red: int, green: int, blue: int, alpha?: int}  $color
	 */
	public function resolveColor(ImageColor|array $color): int
	{}


	/** @param  ImageColor|array{red: int, green: int, blue: int, alpha?: int}  $color */
	private static function normalizeColor(ImageColor|array $color): ImageColor
	{}


	private static function ensureExtension(): void
	{}
}
