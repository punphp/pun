<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use function is_int, is_string;


/**
 * Extends PHP's DateTimeImmutable with strict validation and additional factory methods.
 * Invalid dates and times are rejected with an exception instead of being silently adjusted.
 * All modifications return a new instance, the original object never changes.
 */
class DateTimeImmutable extends \DateTimeImmutable implements \JsonSerializable
{
	/** matches relative sub-day parts (minutes, seconds, ...) that must be applied in UTC to be DST-safe */
	private const RelativePattern = '/[+-]?\s*\d+\s+((microsecond|millisecond|[mµu]sec)s?|[mµ]s|sec(ond)?s?|min(ute)?s?|hours?)(\s+ago)?\b/iu';


	/**
	 * Creates a DateTimeImmutable object from a string, UNIX timestamp, or other DateTimeInterface object.
	 * @throws \Exception if the date and time are not valid.
	 */
	public static function from(string|int|\DateTimeInterface|null $time): static
	{}


	/**
	 * Creates DateTimeImmutable object.
	 * @throws \Exception if the date and time are not valid.
	 */
	public static function fromParts(
		int $year,
		int $month,
		int $day,
		int $hour = 0,
		int $minute = 0,
		float $second = 0.0,
	): static
	{}


	/**
	 * Returns a new DateTimeImmutable object formatted according to the specified format.
	 */
	public static function createFromFormat(
		string $format,
		string $datetime,
		string|\DateTimeZone|null $timezone = null,
	): static|false
	{}


	public function __construct(string $datetime = 'now', ?\DateTimeZone $timezone = null)
	{}


	public function modify(string $modifier): static
	{}


	public function setDate(int $year, int $month, int $day): static
	{}


	public function setTime(int $hour, int $minute, int $second = 0, int $microsecond = 0): static
	{}


	/**
	 * Splits the input into absolute and relative parts and returns the resulting instant. Relative sub-day
	 * parts are applied in UTC so that crossing a DST boundary does not shift the result.
	 */
	private static function resolve(?\DateTimeInterface $base, string $input, ?\DateTimeZone $timezone): \DateTimeImmutable
	{}


	/**
	 * Returns JSON representation in ISO 8601 (used by JavaScript).
	 */
	public function jsonSerialize(): string
	{}


	/**
	 * Returns the date and time in the format 'Y-m-d H:i:s'.
	 */
	public function __toString(): string
	{}


	private static function handleErrors(string $value): void
	{}
}
