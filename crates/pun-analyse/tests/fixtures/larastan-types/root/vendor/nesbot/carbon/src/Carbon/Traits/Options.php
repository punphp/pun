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
use DateTimeInterface;
use Throwable;

/**
 * Trait Options.
 *
 * Embed base methods to change settings of Carbon classes.
 *
 * Depends on the following methods:
 *
 * @method static shiftTimezone($timezone) Set the timezone
 */
trait Options
{
    use StaticOptions;
    use Localization;

    /**
     * Indicates if months should be calculated with overflow.
     * Specific setting.
     */
    protected ?bool $localMonthsOverflow = null;

    /**
     * Indicates if years should be calculated with overflow.
     * Specific setting.
     */
    protected ?bool $localYearsOverflow = null;

    /**
     * Indicates if the strict mode is in use.
     * Specific setting.
     */
    protected ?bool $localStrictModeEnabled = null;

    /**
     * Options for diffForHumans and forHumans methods.
     */
    protected ?int $localHumanDiffOptions = null;

    /**
     * Format to use on string cast.
     *
     * @var string|callable|null
     */
    protected $localToStringFormat = null;

    /**
     * Format to use on JSON serialization.
     *
     * @var string|callable|null
     */
    protected $localSerializer = null;

    /**
     * Instance-specific macros.
     */
    protected ?array $localMacros = null;

    /**
     * Instance-specific generic macros.
     */
    protected ?array $localGenericMacros = null;

    /**
     * Function to call instead of format.
     *
     * @var string|callable|null
     */
    protected $localFormatFunction = null;

    /**
     * Set specific options.
     *  - strictMode: true|false|null
     *  - monthOverflow: true|false|null
     *  - yearOverflow: true|false|null
     *  - humanDiffOptions: int|null
     *  - toStringFormat: string|Closure|null
     *  - toJsonFormat: string|Closure|null
     *  - locale: string|null
     *  - timezone: \DateTimeZone|string|int|null
     *  - macros: array|null
     *  - genericMacros: array|null
     *
     * @param array $settings
     *
     * @return $this|static
     */
    public function settings(array $settings): static
    {}

    /**
     * Returns current local settings.
     */
    public function getSettings(): array
    {}

    /**
     * Show truthy properties on var_dump().
     */
    public function __debugInfo(): array
    {}

    protected function isLocalStrictModeEnabled(): bool
    {}

    protected function addExtraDebugInfos(array &$infos): void
    {}
}
