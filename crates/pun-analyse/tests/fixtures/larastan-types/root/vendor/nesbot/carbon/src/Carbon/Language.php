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

namespace Carbon;

use JsonSerializable;

class Language implements JsonSerializable
{
    protected static ?array $languagesNames = null;

    protected static ?array $regionsNames = null;

    protected string $id;

    protected string $code;

    protected ?string $variant = null;

    protected ?string $region = null;

    protected ?array $names = null;

    protected ?string $isoName = null;

    protected ?string $nativeName = null;

    public function __construct(string $id)
    {}

    /**
     * Get the list of the known languages.
     *
     * @return array
     */
    public static function all(): array
    {}

    /**
     * Get the list of the known regions.
     *
     * ⚠ ISO 3166-2 short name provided with no warranty, should not
     * be used for any purpose to show official state names.
     */
    public static function regions(): array
    {}

    /**
     * Get both isoName and nativeName as an array.
     */
    public function getNames(): array
    {}

    /**
     * Returns the original locale ID.
     */
    public function getId(): string
    {}

    /**
     * Returns the code of the locale "en"/"fr".
     */
    public function getCode(): string
    {}

    /**
     * Returns the variant code such as cyrl/latn.
     */
    public function getVariant(): ?string
    {}

    /**
     * Returns the variant such as Cyrillic/Latin.
     */
    public function getVariantName(): ?string
    {}

    /**
     * Returns the region part of the locale.
     */
    public function getRegion(): ?string
    {}

    /**
     * Returns the region name for the current language.
     *
     * ⚠ ISO 3166-2 short name provided with no warranty, should not
     * be used for any purpose to show official state names.
     */
    public function getRegionName(): ?string
    {}

    /**
     * Returns the long ISO language name.
     */
    public function getFullIsoName(): string
    {}

    /**
     * Set the ISO language name.
     */
    public function setIsoName(string $isoName): static
    {}

    /**
     * Return the full name of the language in this language.
     */
    public function getFullNativeName(): string
    {}

    /**
     * Set the name of the language in this language.
     */
    public function setNativeName(string $nativeName): static
    {}

    /**
     * Returns the short ISO language name.
     */
    public function getIsoName(): string
    {}

    /**
     * Get the short name of the language in this language.
     */
    public function getNativeName(): string
    {}

    /**
     * Get a string with short ISO name, region in parentheses if applicable, variant in parentheses if applicable.
     */
    public function getIsoDescription(): string
    {}

    /**
     * Get a string with short native name, region in parentheses if applicable, variant in parentheses if applicable.
     */
    public function getNativeDescription(): string
    {}

    /**
     * Get a string with long ISO name, region in parentheses if applicable, variant in parentheses if applicable.
     */
    public function getFullIsoDescription(): string
    {}

    /**
     * Get a string with long native name, region in parentheses if applicable, variant in parentheses if applicable.
     */
    public function getFullNativeDescription(): string
    {}

    /**
     * Returns the original locale ID.
     */
    public function __toString(): string
    {}

    /**
     * Get a string with short ISO name, region in parentheses if applicable, variant in parentheses if applicable.
     */
    public function jsonSerialize(): string
    {}
}
