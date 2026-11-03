<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php82\Random\Engine;

use Random\RandomException;
use Symfony\Polyfill\Php82\NoDynamicProperties;

/**
 * @author Tim Düsterhus <tim@bastelstu.be>
 * @author Anton Smirnov <sandfox@sandfox.me>
 *
 * @internal
 */
class Secure
{
    use NoDynamicProperties;

    public function generate(): string
    {}

    public function __sleep(): array
    {}

    public function __wakeup(): void
    {}

    public function __clone()
    {}
}
