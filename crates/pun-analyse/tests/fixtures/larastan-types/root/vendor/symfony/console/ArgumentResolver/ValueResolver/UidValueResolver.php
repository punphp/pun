<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\ArgumentResolver\ValueResolver;

use Symfony\Component\Console\Attribute\Argument;
use Symfony\Component\Console\Attribute\Option;
use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Exception\InvalidOptionException;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Uid\AbstractUid;

/**
 * Resolves an AbstractUid instance from a Command argument or option.
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
final class UidValueResolver implements ValueResolverInterface
{
    public function resolve(string $argumentName, InputInterface $input, ReflectionMember $member): iterable
    {}

    private function resolveArgument(Argument $argument, mixed $value): ?AbstractUid
    {}

    private function resolveOption(Option $option, mixed $value): ?AbstractUid
    {}
}
