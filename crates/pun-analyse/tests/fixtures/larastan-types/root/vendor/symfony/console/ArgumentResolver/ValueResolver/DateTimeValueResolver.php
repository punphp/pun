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

use Psr\Clock\ClockInterface;
use Symfony\Component\Console\Attribute\MapDateTime;
use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Input\InputInterface;

/**
 * Resolves a \DateTime* instance as a command input argument or option.
 *
 * @author Benjamin Eberlei <kontakt@beberlei.de>
 * @author Tim Goudriaan <tim@codedmonkey.com>
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
final class DateTimeValueResolver implements ValueResolverInterface
{
    public function __construct(
        private readonly ?ClockInterface $clock = null,
    ) {}

    public function resolve(string $argumentName, InputInterface $input, ReflectionMember $member): iterable
    {}

    /**
     * @param class-string<\DateTimeImmutable>|class-string<\DateTime> $class
     */
    private function resolveDate(mixed $value, string $class, ?string $format, string $argumentName): \DateTimeInterface
    {}

    /**
     * Takes the fields that $value leaves out from the clock instead of from the system time.
     *
     * @param class-string<\DateTimeImmutable>|class-string<\DateTime> $class
     */
    private function fillFromClock(string $class, \DateTimeInterface $date, string $value, ?string $format): \DateTimeInterface
    {}
}
