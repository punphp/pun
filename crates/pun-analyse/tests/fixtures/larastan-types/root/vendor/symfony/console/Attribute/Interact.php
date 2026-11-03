<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Attribute;

use Symfony\Component\Console\Exception\LogicException;

#[\Attribute(\Attribute::TARGET_METHOD)]
class Interact implements InteractiveAttributeInterface
{
    private \ReflectionMethod $method;

    /**
     * @internal
     */
    public static function tryFrom(\ReflectionMethod $method): ?self
    {}

    /**
     * @internal
     */
    public function getFunction(object $instance): \ReflectionFunction
    {}
}
