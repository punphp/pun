<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Translation;

use Symfony\Component\Translation\Exception\InvalidArgumentException;

/**
 * Derives fallback locales based on ICU parent locale information, by shortening locale
 * sub tags and ultimately by going through a list of configured fallback locales.
 *
 * Also provides locale string validation.
 *
 * @author Matthias Pigulla <mp@webfactory.de>
 */
final class LocaleFallbackProvider
{
    /**
     * @param string[] $localeFallbacks List of fallback locales to add _after_ the ones derived from ICU information
     *
     * @throws InvalidArgumentException If a locale contains invalid characters
     */
    public function __construct(
        private array $localeFallbacks = [],
    ) {}

    /**
     * @return string[]
     */
    public function computeFallbackLocales(string $locale): array
    {}

    /**
     * Asserts that the locale is valid, throws an Exception if not.
     *
     * @throws InvalidArgumentException If the locale contains invalid characters
     */
    public static function validateLocale(string $locale): void
    {}
}
