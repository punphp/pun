<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php82;

/**
 * @author Tim Düsterhus <duesterhus@woltlab.com>
 *
 * @internal
 */
class SensitiveParameterValue
{
    private $value;

    public function __construct($value)
    {}

    public function getValue()
    {}

    public function __debugInfo(): array
    {}

    public function __sleep(): array
    {}

    public function __wakeup(): void
    {}
}
