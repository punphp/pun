<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function strlen;


/**
 * Represent RGB color (0..255) with opacity (0..1).
 */
class ImageColor
{
	/**
	 * Creates a color from RGB components (0..255) and opacity (0..1).
	 */
	public static function rgb(int $red, int $green, int $blue, float $opacity = 1): self
	{}


	/**
	 * Accepts formats #RRGGBB, #RRGGBBAA, #RGB, #RGBA
	 */
	public static function hex(string $hex): self
	{}


	private function __construct(
		public int $red,
		public int $green,
		public int $blue,
		public float $opacity = 1,
	) {}


	/**
	 * Returns GD-compatible color array [R, G, B, alpha].
	 * @return array{int<0, 255>, int<0, 255>, int<0, 255>, int<0, 127>}
	 */
	public function toRGBA(): array
	{}
}
