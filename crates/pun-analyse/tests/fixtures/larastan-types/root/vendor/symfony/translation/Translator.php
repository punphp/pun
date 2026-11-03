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

use Symfony\Component\Config\ConfigCacheFactory;
use Symfony\Component\Config\ConfigCacheFactoryInterface;
use Symfony\Component\Config\ConfigCacheInterface;
use Symfony\Component\Translation\Exception\InvalidArgumentException;
use Symfony\Component\Translation\Exception\NotFoundResourceException;
use Symfony\Component\Translation\Exception\RuntimeException;
use Symfony\Component\Translation\Formatter\IntlFormatterInterface;
use Symfony\Component\Translation\Formatter\MessageFormatter;
use Symfony\Component\Translation\Formatter\MessageFormatterInterface;
use Symfony\Component\Translation\Loader\LoaderInterface;
use Symfony\Contracts\Translation\LocaleAwareInterface;
use Symfony\Contracts\Translation\TranslatableInterface;
use Symfony\Contracts\Translation\TranslatorInterface;

// Help opcache.preload discover always-needed symbols
class_exists(MessageCatalogue::class);

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Translator implements TranslatorInterface, TranslatorBagInterface, LocaleAwareInterface
{
    /**
     * @var MessageCatalogueInterface[]
     */
    protected array $catalogues = [];

    private string $locale;

    /**
     * @var string[]
     */
    private array $fallbackLocales = [];

    /**
     * @var LoaderInterface[]
     */
    private array $loaders = [];

    private array $resources = [];

    private MessageFormatterInterface $formatter;

    private ?ConfigCacheFactoryInterface $configCacheFactory;

    private bool $hasIntlFormatter;

    private LocaleFallbackProvider $localeFallbackProvider;

    /**
     * @var array<string, string|int|float|TranslatableInterface>
     */
    private array $globalParameters = [];

    /**
     * @var array<string, string|int|float>
     */
    private array $globalTranslatedParameters = [];

    /**
     * @throws InvalidArgumentException If a locale contains invalid characters
     */
    public function __construct(
        string $locale,
        ?MessageFormatterInterface $formatter = null,
        private ?string $cacheDir = null,
        private bool $debug = false,
        private array $cacheVary = [],
    ) {}

    public function setConfigCacheFactory(ConfigCacheFactoryInterface $configCacheFactory): void
    {}

    /**
     * Adds a Loader.
     *
     * @param string $format The name of the loader (@see addResource())
     */
    public function addLoader(string $format, LoaderInterface $loader): void
    {}

    /**
     * Adds a Resource.
     *
     * @param string $format   The name of the loader (@see addLoader())
     * @param mixed  $resource The resource name
     *
     * @throws InvalidArgumentException If the locale contains invalid characters
     */
    public function addResource(string $format, mixed $resource, string $locale, ?string $domain = null): void
    {}

    public function setLocale(string $locale): void
    {}

    public function getLocale(): string
    {}

    /**
     * Sets the fallback locales.
     *
     * @param string[] $locales
     *
     * @throws InvalidArgumentException If a locale contains invalid characters
     */
    public function setFallbackLocales(array $locales): void
    {}

    /**
     * @internal
     */
    public function getFallbackLocales(): array
    {}

    public function addGlobalParameter(string $id, string|int|float|TranslatableInterface $value): void
    {}

    public function getGlobalParameters(): array
    {}

    public function trans(?string $id, array $parameters = [], ?string $domain = null, ?string $locale = null): string
    {}

    public function getCatalogue(?string $locale = null): MessageCatalogueInterface
    {}

    public function getCatalogues(): array
    {}

    /**
     * Gets the loaders.
     *
     * @return LoaderInterface[]
     */
    protected function getLoaders(): array
    {}

    protected function loadCatalogue(string $locale): void
    {}

    protected function initializeCatalogue(string $locale): void
    {}

    private function initializeCacheCatalogue(string $locale): void
    {}

    private function dumpCatalogue(string $locale, ConfigCacheInterface $cache): void
    {}

    private function getFallbackContent(MessageCatalogue $catalogue): string
    {}

    private function getCatalogueCachePath(string $locale): string
    {}

    /**
     * @internal
     */
    protected function doLoadCatalogue(string $locale): void
    {}

    private function loadFallbackCatalogues(string $locale): void
    {}

    protected function computeFallbackLocales(string $locale): array
    {}

    /**
     * Asserts that the locale is valid, throws an Exception if not.
     *
     * @throws InvalidArgumentException If the locale contains invalid characters
     */
    protected function assertValidLocale(string $locale): void
    {}

    /**
     * Provides the ConfigCache factory implementation, falling back to a
     * default implementation if necessary.
     */
    private function getConfigCacheFactory(): ConfigCacheFactoryInterface
    {}

    private function getAllMessages(MessageCatalogueInterface $catalogue): array
    {}
}
