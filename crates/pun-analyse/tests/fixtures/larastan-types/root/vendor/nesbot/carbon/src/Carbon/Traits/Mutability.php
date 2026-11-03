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

use Carbon\Carbon;
use Carbon\CarbonImmutable;

/**
 * Trait Mutability.
 *
 * Utils to know if the current object is mutable or immutable and convert it.
 */
trait Mutability
{
    use Cast;

    /**
     * Returns true if the current class/instance is mutable.
     */
    public static function isMutable(): bool
    {}

    /**
     * Returns true if the current class/instance is immutable.
     */
    public static function isImmutable(): bool
    {}

    /**
     * Return a mutable copy of the instance.
     */
    public function toMutable(): Carbon
    {}

    /**
     * Return an immutable copy of the instance.
     */
    public function toImmutable(): CarbonImmutable
    {}
}
