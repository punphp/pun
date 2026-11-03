<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function is_int;
use const JSON_HEX_AMP, JSON_HEX_APOS, JSON_HEX_QUOT, JSON_HEX_TAG, JSON_PRESERVE_ZERO_FRACTION, JSON_UNESCAPED_SLASHES;


/**
 * JSON encoder and decoder.
 */
final class Json
{
	use Nette\StaticClass;

	/** @deprecated use Json::decode(..., forceArrays: true) */
	public const FORCE_ARRAY = JSON_OBJECT_AS_ARRAY;

	/** @deprecated use Json::encode(..., pretty: true) */
	public const PRETTY = JSON_PRETTY_PRINT;

	/** @deprecated use Json::encode(..., asciiSafe: true) */
	public const ESCAPE_UNICODE = 1 << 19;


	/**
	 * Converts value to JSON format. Use $pretty for formatted output, $asciiSafe for ASCII-only output,
	 * $htmlSafe for HTML-safe output, and $forceObjects to encode non-associative arrays as objects.
	 * @throws JsonException
	 */
	public static function encode(
		mixed $value,
		bool|int $pretty = false,
		bool $asciiSafe = false,
		bool $htmlSafe = false,
		bool $forceObjects = false,
	): string
	{}


	/**
	 * Decodes a JSON string to a PHP value. Use $forceArrays to decode objects as arrays.
	 * @throws JsonException
	 */
	public static function decode(string $json, bool|int $forceArrays = false): mixed
	{}
}
