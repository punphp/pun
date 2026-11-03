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

use Carbon\FactoryImmutable;
use Symfony\Contracts\Translation\TranslatorInterface;

/**
 * Static config for localization.
 */
trait StaticLocalization
{
    /**
     * @deprecated To avoid conflict between different third-party libraries, static setters should not be used.
     *             You should rather use the ->settings() method.
     * @see settings
     */
    public static function setHumanDiffOptions(int $humanDiffOptions): void
    {}

    /**
     * @deprecated To avoid conflict between different third-party libraries, static setters should not be used.
     *             You should rather use the ->settings() method.
     * @see settings
     */
    public static function enableHumanDiffOption(int $humanDiffOption): void
    {}

    /**
     * @deprecated To avoid conflict between different third-party libraries, static setters should not be used.
     *             You should rather use the ->settings() method.
     * @see settings
     */
    public static function disableHumanDiffOption(int $humanDiffOption): void
    {}

    /**
     * Return default humanDiff() options (merged flags as integer).
     */
    public static function getHumanDiffOptions(): int
    {}

    /**
     * Set the default translator instance to use.
     *
     * @param TranslatorInterface $translator
     *
     * @return void
     */
    public static function setTranslator(TranslatorInterface $translator): void
    {}

    /**
     * Initialize the default translator instance if necessary.
     */
    public static function getTranslator(): TranslatorInterface
    {}
}
