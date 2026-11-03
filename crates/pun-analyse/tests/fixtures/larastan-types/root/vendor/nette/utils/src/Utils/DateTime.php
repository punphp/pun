<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use function is_string;


/**
 * Extends PHP's DateTime with strict validation and additional factory methods.
 */
class DateTime extends \DateTime implements \JsonSerializable
{
	/** minute in seconds */
	public const MINUTE = 60;

	/** hour in seconds */
	public const HOUR = 60 * self::MINUTE;

	/** day in seconds */
	public const DAY = 24 * self::HOUR;

	/** week in seconds */
	public const WEEK = 7 * self::DAY;

	/** average month in seconds */
	public const MONTH = 2_629_800;

	/** average year in seconds */
	public const YEAR = 31_557_600;


	/**
	 * Creates a DateTime object from a string, UNIX timestamp, or other DateTimeInterface object.
	 * @throws \Exception if the date and time are not valid.
	 */
	public static function from(string|int|\DateTimeInterface|null $time): static
	{}


	/**
	 * Creates DateTime object.
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
	 * Returns a new DateTime object formatted according to the specified format.
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
	 * Converts a relative time string (e.g. '10 minut') to seconds.
	 */
	public static function relativeToSeconds(string $relativeTime): int
	{}


	private function apply(string $datetime, ?\DateTimeZone $timezone = null, bool $ctr = false): void
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


	/**
	 * Returns a modified copy of the object. Use (clone $dt)->modify(...) for better type safety.
	 */
	public function modifyClone(string $modify = ''): static
	{}


	private function handleErrors(string $value): void
	{}
}
