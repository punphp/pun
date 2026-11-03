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

/**
 * Trait Macros.
 *
 * Allows users to register macros within the Carbon class.
 */
trait Macro
{
    use Mixin;

    /**
     * Register a custom macro.
     *
     * Pass null macro to remove it.
     *
     * @example
     * ```
     * $userSettings = [
     *   'locale' => 'pt',
     *   'timezone' => 'America/Sao_Paulo',
     * ];
     * Carbon::macro('userFormat', function () use ($userSettings) {
     *   return $this->copy()->locale($userSettings['locale'])->tz($userSettings['timezone'])->calendar();
     * });
     * echo Carbon::yesterday()->hours(11)->userFormat();
     * ```
     *
     * @param-closure-this static $macro
     */
    public static function macro(string $name, ?callable $macro): void
    {}

    /**
     * Remove all macros and generic macros.
     */
    public static function resetMacros(): void
    {}

    /**
     * Register a custom macro.
     *
     * @param callable $macro
     * @param int      $priority marco with higher priority is tried first
     *
     * @return void
     */
    public static function genericMacro(callable $macro, int $priority = 0): void
    {}

    /**
     * Checks if macro is registered globally.
     *
     * @param string $name
     *
     * @return bool
     */
    public static function hasMacro(string $name): bool
    {}

    /**
     * Get the raw callable macro registered globally for a given name.
     */
    public static function getMacro(string $name): ?callable
    {}

    /**
     * Checks if macro is registered globally or locally.
     */
    public function hasLocalMacro(string $name): bool
    {}

    /**
     * Get the raw callable macro registered globally or locally for a given name.
     */
    public function getLocalMacro(string $name): ?callable
    {}
}
