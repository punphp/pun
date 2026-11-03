<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Exception;

/**
 * Represents an incorrect option name or value typed in the console.
 *
 * @author Jérôme Tamarelle <jerome@tamarelle.net>
 */
class InvalidOptionException extends \InvalidArgumentException implements ExceptionInterface
{
    /**
     * @internal
     */
    public static function fromEnumValue(string $name, string $value, array|\Closure $suggestedValues): self
    {}

    /**
     * @internal
     */
    public static function fromInvalidType(string $name, string $value, string $type): self
    {}
}
