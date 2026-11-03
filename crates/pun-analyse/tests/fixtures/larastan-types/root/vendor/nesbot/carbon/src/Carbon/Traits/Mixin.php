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
use Carbon\CarbonInterval;
use Carbon\CarbonPeriod;
use Closure;
use Generator;
use ReflectionClass;
use ReflectionException;
use ReflectionMethod;
use ReflectionNamedType;
use Throwable;

/**
 * Trait Mixin.
 *
 * Allows mixing in entire classes with multiple macros.
 */
trait Mixin
{
    /**
     * Stack of macro instance contexts.
     */
    protected static array $macroContextStack = [];

    /**
     * Mix another object into the class.
     *
     * @example
     * ```
     * Carbon::mixin(new class {
     *   public function addMoon() {
     *     return function () {
     *       return $this->addDays(30);
     *     };
     *   }
     *   public function subMoon() {
     *     return function () {
     *       return $this->subDays(30);
     *     };
     *   }
     * });
     * $fullMoon = Carbon::create('2018-12-22');
     * $nextFullMoon = $fullMoon->addMoon();
     * $blackMoon = Carbon::create('2019-01-06');
     * $previousBlackMoon = $blackMoon->subMoon();
     * echo "$nextFullMoon\n";
     * echo "$previousBlackMoon\n";
     * ```
     *
     * @throws ReflectionException
     */
    public static function mixin(object|string $mixin): void
    {}

    /**
     * @throws ReflectionException
     */
    private static function loadMixinClass(object|string $mixin): void
    {}

    private static function cannotBeAMixinMethod(ReflectionMethod $method): bool
    {}

    private static function loadMixinTrait(string $trait): void
    {}

    private static function getAnonymousClassCodeForTrait(string $trait): string
    {}

    private static function getMixableMethods(self $context): Generator
    { yield; }

    /**
     * Stack a Carbon context from inside calls of self::this() and execute a given action.
     */
    protected static function bindMacroContext(?self $context, callable $callable): mixed
    {}

    /**
     * Return the current context from inside a macro callee or a null if static.
     */
    protected static function context(): ?static
    {}

    /**
     * Return the current context from inside a macro callee or a new one if static.
     */
    protected static function this(): static
    {}
}
