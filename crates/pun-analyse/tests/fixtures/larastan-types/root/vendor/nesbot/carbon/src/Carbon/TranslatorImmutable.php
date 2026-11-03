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

use Carbon\Exceptions\ImmutableException;
use Symfony\Component\Config\ConfigCacheFactoryInterface;
use Symfony\Component\Translation\Formatter\MessageFormatterInterface;

class TranslatorImmutable extends Translator
{
    private bool $constructed = false;

    public function __construct($locale, ?MessageFormatterInterface $formatter = null, $cacheDir = null, $debug = false)
    {}

    /**
     * @codeCoverageIgnore
     */
    public function setDirectories(array $directories): static
    {}

    public function setLocale($locale): void
    {}

    /**
     * @codeCoverageIgnore
     */
    public function setMessages(string $locale, array $messages): static
    {}

    /**
     * @codeCoverageIgnore
     */
    public function setTranslations(array $messages): static
    {}

    /**
     * @codeCoverageIgnore
     */
    public function setConfigCacheFactory(ConfigCacheFactoryInterface $configCacheFactory): void
    {}

    public function resetMessages(?string $locale = null): bool
    {}

    /**
     * @codeCoverageIgnore
     */
    public function setFallbackLocales(array $locales): void
    {}

    private function disallowMutation($method)
    {}
}
