<?php

declare(strict_types=1);

/**
 * This file is part of the Carbon package.
 *
 * (c) Brian Nesbitt <brian@nesbot.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Carbon\Traits;

use Carbon\CarbonInterface;
use Carbon\Exceptions\InvalidFormatException;
use Carbon\Translator;
use InvalidArgumentException;
use Symfony\Component\Translation\TranslatorBagInterface;
use Symfony\Contracts\Translation\LocaleAwareInterface;
use Symfony\Contracts\Translation\TranslatorInterface;

/**
 * Trait Eras.
 *
 * Era (moment.js style) helpers, used to read and describe the eras of the current locale.
 */
trait Eras
{
    /**
     * Return the era of the current locale the current date is in (null if none).
     *
     * Eras follow moment.js: a list of ['since', 'until', 'offset', 'name', 'narrow', 'abbr'] where since
     * and until are 'YYYY-MM-DD' dates (or INF/-INF for until) and since may be after until for eras counting
     * years backward (such as BC).
     *
     * @return array{name: string|null, narrow: string|null, abbr: string|null, offset: int, sinceYear: int, untilYear: int|float, direction: int}|null
     */
    private function getEra(): ?array
    {}

    /**
     * @param array<string, mixed> $era
     *
     * @return array{name: string|null, narrow: string|null, abbr: string|null, offset: int, sinceYear: int, untilYear: int|float, direction: int, since: int|float, until: int|float}
     */
    private static function describeEra(array $era): array
    {}

    /**
     * Return the "eras" of the current locale, taken as a whole from the first locale of the fallback chain
     * defining them, so a locale with fewer eras does not get the extra ones of its fallback.
     * Like in moment.js, Anno Domini / Before Christ are used if no locale defines eras.
     *
     * @return array<int, array<string, mixed>>
     */
    private function getEras(): array
    {}

    /**
     * @return array<int, array<string, mixed>>
     */
    private static function getDefaultEras(): array
    {}

    /**
     * Parse an era limit ('YYYY-MM-DD', the year may be negative, or INF/-INF) into its year and a sortable value.
     *
     * @return array{0: int|float, 1: int|float}
     */
    private static function parseEraDate(mixed $date): array
    {}

    /**
     * Replace the era tokens of an ISO format (and what they match in $time) by a Gregorian year, so the result
     * can be parsed by rawCreateFromFormat(). The era year is converted before parsing so a leap day is checked
     * against the right year (BE 2567-02-29 is valid, but CE 2567-02-29 is not).
     *
     * @throws InvalidFormatException
     *
     * @return array{0: string, 1: string} format and time without era tokens
     */
    private static function resolveIsoFormatEras(string $format, string $time, ?string $locale, ?TranslatorInterface $translator): array
    {}

    /**
     * Pattern matching the era names (or abbreviations, or narrow names depending on the token) of the given eras.
     *
     * @param array<int, array<string, mixed>> $eras
     */
    private static function getEraNamesPattern(array $eras, string $token): string
    {}

    /**
     * @param array<int, array<string, mixed>> $eras
     *
     * @return array<string, mixed>
     */
    private static function findEraByName(array $eras, string $token, string $name): array
    {}

    private static function getEraField(string $token): string
    {}

    /**
     * Pattern used to locate the era in a string, only to read what is around the era tokens (PHP does the parsing).
     */
    private static function getIsoTokenPattern(string $token): string
    {}
}
