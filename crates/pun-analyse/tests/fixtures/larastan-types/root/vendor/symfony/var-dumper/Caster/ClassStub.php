<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Represents a PHP class identifier.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class ClassStub extends ConstStub
{
    /**
     * @param string   $identifier A PHP identifier, e.g. a class, method, interface, etc. name
     * @param callable $callable   The callable targeted by the identifier when it is ambiguous or not a real PHP identifier
     */
    public function __construct(string $identifier, callable|array|string|null $callable = null)
    {}

    public static function wrapCallable(mixed $callable): mixed
    {}
}
