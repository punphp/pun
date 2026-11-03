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
use Carbon\Exceptions\InvalidTypeException;
use Carbon\Exceptions\NotLocaleAwareException;
use Carbon\Language;
use Carbon\Translator;
use Carbon\TranslatorStrongTypeInterface;
use Closure;
use Symfony\Component\Translation\TranslatorBagInterface;
use Symfony\Contracts\Translation\LocaleAwareInterface;
use Symfony\Contracts\Translation\TranslatorInterface;

/**
 * Trait Localization.
 *
 * Embed default and locale translators and translation base methods.
 */
trait Localization
{
    use StaticLocalization;

    /**
     * Specific translator of the current instance.
     */
    protected ?TranslatorInterface $localTranslator = null;

    /**
     * Return true if the current instance has its own translator.
     */
    public function hasLocalTranslator(): bool
    {}

    /**
     * Get the translator of the current instance or the default if none set.
     */
    public function getLocalTranslator(): TranslatorInterface
    {}

    /**
     * Set the translator for the current instance.
     */
    public function setLocalTranslator(TranslatorInterface $translator): self
    {}

    /**
     * Returns raw translation message for a given key.
     *
     * @param TranslatorInterface|null $translator the translator to use
     * @param string                   $key        key to find
     * @param string|null              $locale     current locale used if null
     * @param string|null              $default    default value if translation returns the key
     *
     * @return string|Closure|null
     */
    public static function getTranslationMessageWith($translator, string $key, ?string $locale = null, ?string $default = null)
    {}

    /**
     * Returns raw translation message for a given key.
     *
     * @param string              $key        key to find
     * @param string|null         $locale     current locale used if null
     * @param string|null         $default    default value if translation returns the key
     * @param TranslatorInterface $translator an optional translator to use
     *
     * @return string
     */
    public function getTranslationMessage(string $key, ?string $locale = null, ?string $default = null, $translator = null)
    {}

    /**
     * Translate using translation string or callback available.
     *
     * @param TranslatorInterface $translator an optional translator to use
     * @param string              $key        key to find
     * @param array               $parameters replacement parameters
     * @param int|float|null      $number     number if plural
     *
     * @return string
     */
    public static function translateWith(TranslatorInterface $translator, string $key, array $parameters = [], $number = null): string
    {}

    /**
     * Translate using translation string or callback available.
     *
     * @param string                   $key        key to find
     * @param array                    $parameters replacement parameters
     * @param string|int|float|null    $number     number if plural
     * @param TranslatorInterface|null $translator an optional translator to use
     * @param bool                     $altNumbers pass true to use alternative numbers
     *
     * @return string
     */
    public function translate(
        string $key,
        array $parameters = [],
        string|int|float|null $number = null,
        ?TranslatorInterface $translator = null,
        bool $altNumbers = false,
    ): string {}

    /**
     * Returns the alternative number for a given integer if available in the current locale.
     *
     * @param int $number
     *
     * @return string
     */
    public function translateNumber(int $number): string
    {}

    /**
     * Translate a time string from a locale to an other.
     *
     * @param string      $timeString date/time/duration string to translate (may also contain English)
     * @param string|null $from       input locale of the $timeString parameter (`Carbon::getLocale()` by default)
     * @param string|null $to         output locale of the result returned (`"en"` by default)
     * @param int         $mode       specify what to translate with options:
     *                                - CarbonInterface::TRANSLATE_ALL (default)
     *                                - CarbonInterface::TRANSLATE_MONTHS
     *                                - CarbonInterface::TRANSLATE_DAYS
     *                                - CarbonInterface::TRANSLATE_UNITS
     *                                - CarbonInterface::TRANSLATE_MERIDIEM
     *                                You can use pipe to group: CarbonInterface::TRANSLATE_MONTHS | CarbonInterface::TRANSLATE_DAYS
     *
     * @return string
     */
    public static function translateTimeString(
        string $timeString,
        ?string $from = null,
        ?string $to = null,
        int $mode = CarbonInterface::TRANSLATE_ALL,
    ): string {}

    private static function getMatchingWordIndexes(array $fromTranslations, string $search): array
    {}

    /**
     * Translate a time string from the current locale (`$date->locale()`) to another one.
     *
     * @param string      $timeString time string to translate
     * @param string|null $to         output locale of the result returned ("en" by default)
     *
     * @return string
     */
    public function translateTimeStringTo(string $timeString, ?string $to = null): string
    {}

    /**
     * Get/set the locale for the current instance.
     *
     * @param string|null $locale
     * @param string      ...$fallbackLocales
     *
     * @return ($locale is null ? string : static)
     */
    public function locale(?string $locale = null, string ...$fallbackLocales): static|string
    {}

    /**
     * Get the current translator locale.
     *
     * @return string
     */
    public static function getLocale(): string
    {}

    /**
     * Set the current translator locale and indicate if the source locale file exists.
     * Pass 'auto' as locale to use the closest language to the current LC_TIME locale.
     *
     * @param string $locale locale ex. en
     */
    public static function setLocale(string $locale): void
    {}

    /**
     * Set the fallback locale.
     *
     * @see https://symfony.com/doc/current/components/translation.html#fallback-locales
     *
     * @param string $locale
     */
    public static function setFallbackLocale(string $locale): void
    {}

    /**
     * Get the fallback locale.
     *
     * @see https://symfony.com/doc/current/components/translation.html#fallback-locales
     */
    public static function getFallbackLocale(): ?string
    {}

    /**
     * Set the current locale to the given, execute the passed function, reset the locale to previous one,
     * then return the result of the closure (or null if the closure was void).
     *
     * @param string   $locale locale ex. en
     * @param callable $func
     *
     * @return mixed
     */
    public static function executeWithLocale(string $locale, callable $func): mixed
    {}

    /**
     * Returns true if the given locale is internally supported and has short-units support.
     * Support is considered enabled if either year, day or hour has a short variant translated.
     *
     * @param string $locale locale ex. en
     *
     * @return bool
     */
    public static function localeHasShortUnits(string $locale): bool
    {}

    /**
     * Returns true if the given locale is internally supported and has diff syntax support (ago, from now, before, after).
     * Support is considered enabled if the 4 sentences are translated in the given locale.
     *
     * @param string $locale locale ex. en
     *
     * @return bool
     */
    public static function localeHasDiffSyntax(string $locale): bool
    {}

    /**
     * Returns true if the given locale is internally supported and has words for 1-day diff (just now, yesterday, tomorrow).
     * Support is considered enabled if the 3 words are translated in the given locale.
     *
     * @param string $locale locale ex. en
     *
     * @return bool
     */
    public static function localeHasDiffOneDayWords(string $locale): bool
    {}

    /**
     * Returns true if the given locale is internally supported and has words for 2-days diff (before yesterday, after tomorrow).
     * Support is considered enabled if the 2 words are translated in the given locale.
     *
     * @param string $locale locale ex. en
     *
     * @return bool
     */
    public static function localeHasDiffTwoDayWords(string $locale): bool
    {}

    /**
     * Returns true if the given locale is internally supported and has period syntax support (X times, every X, from X, to X).
     * Support is considered enabled if the 4 sentences are translated in the given locale.
     *
     * @param string $locale locale ex. en
     *
     * @return bool
     */
    public static function localeHasPeriodSyntax($locale)
    {}

    /**
     * Returns the list of internally available locales and already loaded custom locales.
     * (It will ignore custom translator dynamic loading.)
     *
     * @return array
     */
    public static function getAvailableLocales(): array
    {}

    /**
     * Returns list of Language object for each available locale. This object allow you to get the ISO name, native
     * name, region and variant of the locale.
     *
     * @return Language[]
     */
    public static function getAvailableLocalesInfo(): array
    {}

    /**
     * Get the locale of a given translator.
     *
     * If null or omitted, current local translator is used.
     * If no local translator is in use, current global translator is used.
     */
    protected function getTranslatorLocale($translator = null): ?string
    {}

    /**
     * Throw an error if passed object is not LocaleAwareInterface.
     *
     * @param LocaleAwareInterface|null $translator
     *
     * @return LocaleAwareInterface|null
     */
    protected static function getLocaleAwareTranslator($translator = null)
    {}

    /**
     * @param mixed                                                    $translator
     * @param \Symfony\Component\Translation\MessageCatalogueInterface $catalogue
     *
     * @return mixed
     */
    private static function getFromCatalogue($translator, $catalogue, string $id, string $domain = 'messages')
    {}

    /**
     * Return the word cleaned from its translation codes.
     *
     * @param string $word
     *
     * @return string
     */
    private static function cleanWordFromTranslationString($word)
    {}

    /**
     * Translate a list of words.
     *
     * @param string[] $keys     keys to translate.
     * @param string[] $messages messages bag handling translations.
     * @param string   $key      'to' (to get the translation) or 'from' (to get the detection RegExp pattern).
     *
     * @return string[]
     */
    private static function translateWordsByKeys($keys, $messages, $key): array
    {}

    /**
     * Get an array of translations based on the current date.
     *
     * @param callable $translation
     * @param int      $length
     * @param string   $timeString
     *
     * @return string[]
     */
    private static function getTranslationArray($translation, $length, $timeString): array
    {}

    private static function replaceOrdinalWords(string $timeString, array $ordinalWords): string
    {}
}
